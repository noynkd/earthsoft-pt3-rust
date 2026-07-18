use crate::buffer;
use earthsoft_sdk::pt3;

// =============================================================================
// BufferSpec
// =============================================================================

#[derive(Debug, Clone, Copy, Default)]
struct BufferSpec {
    block_size: u32,
    block_count: u32,
    direction: pt3::TransferDirection,
}

// =============================================================================
// BufferStorage
// =============================================================================

#[derive(Debug)]
struct BufferStorage {
    block_size: u32,
    block_count: u32,
    buffer: buffer::MemoryBuffer,
    physical_blocks: Vec<buffer::PhysicalBlock>
}

impl BufferStorage {
    fn new(device: std::sync::Arc<pt3::Device>, spec: BufferSpec) -> Result<Self, pt3::Error> {
        let size = spec.block_size * spec.block_count;

        let mut buffer = buffer::MemoryBuffer::new(size as usize)
            .map_err(|e| {
                eprintln!("buffer::MemoryBuffer::new() に失敗しました: {:?}", e);
                pt3::Error::InternalError
            })?;
        let slice = buffer.slice_mut();
        if slice.len() == 0 {
            return Err(pt3::Error::OutOfMemory);
        }

        let mut blocks = Vec::with_capacity(spec.block_count as usize);
        let mut offset = 0;

        for _ in 0..spec.block_count {
            let chunk = &mut slice[offset..][..(spec.block_size as usize)];
            let block = buffer::PhysicalBlock::new(device.clone(), chunk, spec.direction)
                .inspect_err(|e| {
                    eprintln!("buffer::PhysicalBlock::new() に失敗しました。: {:?}", e);
                })?;
            blocks.push(block);

            offset += spec.block_size as usize;
        }

        Ok(Self {
            block_size: spec.block_size,
            block_count: spec.block_count,
            buffer,
            physical_blocks: blocks,
        })
    }
}

// =============================================================================
// PageBuffer
// =============================================================================

#[derive(Debug, Default)]
pub struct PageBuffer {
    spec: BufferSpec,
    storage: Option<BufferStorage>,
}

impl PageBuffer {
    const PAGE_SIZE: usize = 4096;
    const DESCRIPTOR_SIZE: usize = 20;

    pub fn new(buffer_size: u32) -> Self {
        let descriptors_per_page = Self::PAGE_SIZE / Self::DESCRIPTOR_SIZE;
        let total_ts_pages = buffer_size as usize / Self::PAGE_SIZE;
        let requires_page_count = (total_ts_pages + descriptors_per_page - 1) / descriptors_per_page;
        let requires_page_size = Self::PAGE_SIZE * requires_page_count;

        let spec = BufferSpec {
            block_size:  requires_page_size as u32,
            block_count: 1,
            direction:   pt3::TransferDirection::Read,
        };

        Self {
            spec,
            storage: None,
        }
    }

    pub fn allocate(&mut self, device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
        self.storage.replace(BufferStorage::new(device.clone(), self.spec)?);

        Ok(())
    }

    pub fn sync_cpu(&self) -> Result<(), pt3::Error> {
        let Some(storage) = self.storage.as_ref() else {
            return Err(pt3::Error::BufferNotAllocated);
        };

        let Some(block) = storage.physical_blocks.first() else {
            eprintln!("buffer::PageBuffer::sync_io(): インデックスが範囲外です.");
            return Err(pt3::Error::InternalError)
        };

        block
            .sync_cpu()
            .inspect_err(|e| {
                eprintln!("buffer::PhysicalBlock::sync_cpu() に失敗しました:{:?}", e);
            })?;

        Ok(())
    }

    pub fn get_descriptor_address(&self) -> u64 {
        let Some(storage) = self.storage.as_ref() else {
            return 0;
        };

        let Some(block) = storage.physical_blocks.first() else {
            return 0;
        };

        let Some(info) = block.get_buffer_info().first() else {
            return 0;
        };

        return info.address;
    }

    pub fn build_page_descriptor(&mut self, ts_buffer: &TsBuffer, loop_back: bool) -> Result<(), pt3::Error> {
        let Some(storage) = self.storage.as_mut() else {
            return Err(pt3::Error::BufferNotAllocated);
        };

        let buffer = storage.buffer.slice_mut();
        if buffer.len() == 0 {
            eprintln!("buffer::PageBuffer::build_page_descriptor(): バッファがありません.");
            return Err(pt3::Error::InternalError);
        }

        let Some(block) = storage.physical_blocks.first() else {
            eprintln!("buffer::PageBuffer::build_page_descriptor(): インデックスが範囲外です.");
            return Err(pt3::Error::InternalError)
        };

        let pages = block.get_buffer_info();
        let total_size = pages.iter()
            .map(|page| { page.size as usize })
            .sum::<usize>();

        // 仮想バッファが物理バッファに対して十分なサイズを持っていない
        if buffer.len() < total_size {
            return Err(pt3::Error::OutOfMemory);
        }

        let Some(ts_storage) = ts_buffer.storage.as_ref() else {
            return Err(pt3::Error::BufferNotAllocated);
        };

        let mut buffer_index = 0;
        let mut page_index = 0;
        let mut address = pages[0].address;
        let mut remain = pages[0].size as usize;
        let first_address = pages[0].address;
        let mut previous_index = usize::MAX;

        for ts_block in &ts_storage.physical_blocks {
            for ts_buffer_info in ts_block.get_buffer_info() {
                let total_pages = ts_buffer_info.size as usize / Self::PAGE_SIZE;
                let mut current_block_address = ts_buffer_info.address;

                for _ in 0..total_pages {
                    while remain < Self::DESCRIPTOR_SIZE {
                        buffer_index += remain;   // 残りを飛ばす

                        page_index += 1;
                        // 次のページがない場合は書き込むものがなくなるので終了する
                        if pages.len() <= page_index {
                            return Err(pt3::Error::InternalError);
                        }

                        address = pages[page_index].address;
                        remain = pages[page_index].size as usize;
                    }

                    if previous_index != usize::MAX {
                        let next = address;
                        Self::link_descriptor(&mut buffer[previous_index..][..Self::DESCRIPTOR_SIZE], next);
                    }

                    Self::write_descriptor(&mut buffer[buffer_index..][..Self::DESCRIPTOR_SIZE], current_block_address, Self::PAGE_SIZE as u32);

                    previous_index = buffer_index;
                    buffer_index += Self::DESCRIPTOR_SIZE;
                    address += Self::DESCRIPTOR_SIZE as u64;
                    remain -= Self::DESCRIPTOR_SIZE;

                    current_block_address += Self::PAGE_SIZE as u64;
                }
            }
        }

        if previous_index != usize::MAX {
            let next = if loop_back { first_address } else { 1 };
            Self::link_descriptor(&mut buffer[previous_index..][..Self::DESCRIPTOR_SIZE], next);
        }

        Ok(())
    }

    fn write_descriptor(bytes: &mut [u8], address: u64, size: u32) {
        unsafe {
            std::ptr::write_unaligned(
                bytes.as_mut_ptr().cast::<u64>(),
                address | 7);
            std::ptr::write_unaligned(
                bytes.as_mut_ptr().add( 8).cast::<u32>(),
                size | 7);
            std::ptr::write_unaligned(
                bytes.as_mut_ptr().add(12).cast::<u64>(),
                0 | 2);
        }
    }

    fn link_descriptor(bytes: &mut [u8], next: u64) {
        unsafe {
            std::ptr::write_unaligned(
                bytes.as_mut_ptr().add(12).cast::<u64>(),
                next | 2);
        }
    }
}

// =============================================================================
// TsBuffer
// =============================================================================

#[derive(Debug, Default)]
pub struct TsBuffer {
    spec: BufferSpec,
    storage: Option<BufferStorage>,
}

impl TsBuffer {
    pub fn new(block_size: u32, block_count: u32) -> Self {
        let spec = BufferSpec {
            block_size,
            block_count,
            direction: pt3::TransferDirection::Write,
        };

        Self {
            spec,
            storage: None,
        }
    }

    pub fn allocate(&mut self, device: std::sync::Arc<pt3::Device>) -> Result<(), pt3::Error> {
        self.storage.replace(BufferStorage::new(device.clone(), self.spec)?);

        Ok(())
    }

    pub fn sync_cpu(&self, block_index: u32) -> Result<(), pt3::Error> {
        let Some(storage) = self.storage.as_ref() else {
            return Err(pt3::Error::BufferNotAllocated);
        };

        let Some(block) = storage.physical_blocks.get(block_index as usize) else {
            eprintln!("buffer::PageBuffer::sync_io(): インデックスが範囲外です.");
            return Err(pt3::Error::InternalError);
        };

        block.sync_cpu()
            .inspect_err(|e| {
                eprintln!("buffer::PhysicalBlock::sync_cpu() に失敗しました: {:?}", e);
            })?;

        Ok(())
    }

    pub fn sync_io(&self, block_index: u32) -> Result<(), pt3::Error> {     
        let Some(storage) = self.storage.as_ref() else {
            return Err(pt3::Error::BufferNotAllocated);
        };

        let Some(block) = storage.physical_blocks.get(block_index as usize) else {
            eprintln!("buffer::PageBuffer::sync_io(): インデックスが範囲外です.");
            return Err(pt3::Error::InternalError)
        };

        block.sync_io()
            .inspect_err(|e| {
                eprintln!("buffer::PhysicalBlock::sync_io() に失敗しました: {:?}", e);
            })?;

        Ok(())
    }

    pub fn slice_block(&self, block_index: u32) -> &[u8] {
        let Some(storage) = self.storage.as_ref() else {
            return &[];
        };

        if block_index >= storage.block_count {
            return &[];
        }

        let begin = (storage.block_size * block_index) as usize;
        let end = begin + storage.block_size as usize;

        &storage.buffer.slice()[begin..end]
    }

    pub fn slice_block_mut(&mut self, block_index: u32) -> &mut [u8] {
        let Some(storage) = self.storage.as_mut() else {
            return &mut [];
        };

        if block_index >= storage.block_count {
            return &mut [];
        }

        let begin = (storage.block_size * block_index) as usize;
        let end = begin + storage.block_size as usize;

        &mut storage.buffer.slice_mut()[begin..end]
    }
}
