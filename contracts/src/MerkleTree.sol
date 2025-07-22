// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

uint8 constant TREE_DEPTH = 32;
uint256 constant MAX_LEAVES = 1 << TREE_DEPTH;

struct MerkleTree {
    bytes32 root;
    uint32 nextIndex;
    bytes32[TREE_DEPTH] filledSubtree; // left-hand subtree hashes
    bytes32[TREE_DEPTH] zeroHash; // pre-computed default hashes
}

library MerkleTreeLib {
    event LeafInserted(uint256 index, bytes32 leaf);

    /// @notice Create a new Merkle tree
    /// @return tree The new Merkle tree
    function newTree() internal pure returns (MerkleTree memory tree) {
        bytes32 current = bytes32(0);
        for (uint8 i = 0; i < TREE_DEPTH; ++i) {
            current = keccak256(abi.encodePacked(current, current));
            tree.zeroHash[i] = current;
        }
        tree.root = tree.zeroHash[TREE_DEPTH - 1]; // empty tree’s root

        return tree;
    }

    function insert(MerkleTree storage tree, bytes32 leaf) internal returns (uint32 index) {
        index = tree.nextIndex;
        require(index < MAX_LEAVES, "tree full");

        bytes32 currentHash = leaf;
        uint32 idx = index;

        for (uint8 level = 0; level < TREE_DEPTH; ++level) {
            if (idx & 1 == 0) {
                tree.filledSubtree[level] = currentHash;
                currentHash = keccak256(abi.encodePacked(currentHash, tree.zeroHash[level]));
            } else {
                currentHash = keccak256(abi.encodePacked(tree.filledSubtree[level], currentHash));
            }
            idx >>= 1;
        }

        tree.root = currentHash;
        unchecked {
            tree.nextIndex += 1;
        }
    }
}
