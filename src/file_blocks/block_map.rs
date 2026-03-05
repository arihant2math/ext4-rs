use crate::Inode;
use crate::block_index::{FileBlockIndex, FsBlockIndex};
use crate::util::read_u32le;

pub(crate) struct BlockMap {
    direct_blocks: [u32; 12],
    single_indirect_block: u32,
    double_indirect_block: u32,
    triple_indirect_block: u32,
}

impl BlockMap {
    pub(crate) fn initialize() -> Self {
        Self {
            direct_blocks: [0; 12],
            single_indirect_block: 0,
            double_indirect_block: 0,
            triple_indirect_block: 0,
        }
    }

    pub(crate) fn from_inode(inode: &Inode) -> Self {
        let data = inode.inline_data();
        let mut direct_blocks = [0; 12];
        for (i, direct_block) in direct_blocks.iter_mut().enumerate() {
            *direct_block = read_u32le(&data, i * 4);
        }
        let single_indirect_block = read_u32le(&data, 12 * 4);
        let double_indirect_block = read_u32le(&data, 13 * 4);
        let triple_indirect_block = read_u32le(&data, 14 * 4);
        Self {
            direct_blocks,
            single_indirect_block,
            double_indirect_block,
            triple_indirect_block,
        }
    }

    pub(crate) fn to_bytes(&self) -> [u8; 15 * 4] {
        let mut data = [0; 15 * 4];
        for i in 0..12 {
            data[i * 4..(i + 1) * 4]
                .copy_from_slice(&self.direct_blocks[i].to_le_bytes());
        }
        data[12 * 4..13 * 4]
            .copy_from_slice(&self.single_indirect_block.to_le_bytes());
        data[13 * 4..14 * 4]
            .copy_from_slice(&self.double_indirect_block.to_le_bytes());
        data[14 * 4..15 * 4]
            .copy_from_slice(&self.triple_indirect_block.to_le_bytes());
        data
    }

    pub(crate) fn map_block(
        &self,
        file_block_index: FileBlockIndex,
    ) -> FsBlockIndex {
        if file_block_index < 12 {
            self.direct_blocks[file_block_index as usize] as u64
        } else {
            todo!(
                "Handle indirect blocks for file block index {}",
                file_block_index
            );
        }
    }

    pub(crate) fn set_block(
        &mut self,
        file_block_index: FileBlockIndex,
        fs_block_index: FsBlockIndex,
    ) {
        if file_block_index < 12 {
            self.direct_blocks[file_block_index as usize] =
                fs_block_index as u32;
        } else {
            todo!(
                "Handle indirect blocks for file block index {}",
                file_block_index
            );
        }
    }
}
