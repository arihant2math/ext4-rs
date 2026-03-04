#![allow(unused)]

use crate::{Ext4, Ext4Error, Inode, InodeFlags};

use crate::block_index::{FileBlockIndex, FsBlockIndex};
use alloc::vec::Vec;

mod block_map;
pub(crate) mod extent_tree;

pub(crate) enum FileBlocks {
    BlockMap(block_map::BlockMap),
    ExtentTree(extent_tree::ExtentTree),
}

impl FileBlocks {
    pub(crate) fn initialize(
        ext4: Ext4,
        inode: &Inode,
    ) -> Result<Self, Ext4Error> {
        if inode.flags().contains(InodeFlags::EXTENTS) {
            Ok(Self::ExtentTree(extent_tree::ExtentTree::initialize(
                ext4, inode,
            )?))
        } else {
            unimplemented!("Non-extent file blocks are not implemented yet");
        }
    }

    pub(crate) fn from_inode(
        ext4: Ext4,
        inode: &Inode,
    ) -> Result<Self, Ext4Error> {
        if inode.flags().contains(InodeFlags::EXTENTS) {
            Ok(Self::ExtentTree(extent_tree::ExtentTree::from_inode(
                ext4, inode,
            )?))
        } else {
            Ok(Self::BlockMap(block_map::BlockMap::from_inode(inode)))
        }
    }

    pub(crate) fn to_bytes(&self) -> [u8; 60] {
        match self {
            Self::ExtentTree(extent_tree) => extent_tree.to_bytes(),
            Self::BlockMap(block_map) => block_map.to_bytes(),
        }
    }

    pub(crate) async fn get_block(
        &self,
        block_index: FileBlockIndex,
    ) -> Result<Option<FsBlockIndex>, Ext4Error> {
        match self {
            Self::ExtentTree(extent_tree) => {
                extent_tree.get_block(block_index).await
            }
            Self::BlockMap(block_map) => todo!(),
        }
    }
}
