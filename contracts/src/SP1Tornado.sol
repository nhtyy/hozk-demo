// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {MerkleTreeLib, MerkleTree} from "./MerkleTree.sol";
import {ISP1Verifier} from "sp1-contracts/src/ISP1Verifier.sol";

struct ProofCommitment {
    bytes32 nullifierHash;
    bytes32 root;
}

contract SP1Tornado {
    event LeafInserted(bytes32 leaf, uint32 index);

    mapping(bytes32 => bool) public nullifierHash;
    MerkleTree tree;
    ISP1Verifier immutable verifier;
    bytes32 immutable vkey;
    uint256 immutable depositAmount;

    constructor(uint256 _depositAmount, address _verifier, bytes32 _vkey) {
        tree = MerkleTreeLib.newTree();
        depositAmount = _depositAmount;
        verifier = ISP1Verifier(_verifier);
        vkey = _vkey;
    }

    function deposit(bytes32 noteHash) public payable {
        require(msg.value == depositAmount, "Invalid deposit amount");

        uint32 idx = MerkleTreeLib.insert(tree, noteHash);
        emit LeafInserted(noteHash, idx);
    }

    function withdraw(bytes32 _nullifierHash, bytes calldata proof) public {
        require(!nullifierHash[_nullifierHash], "Nullifier already spent");
        nullifierHash[_nullifierHash] = true;

        ProofCommitment memory commitment = ProofCommitment({nullifierHash: _nullifierHash, root: tree.root});

        // Panics if the proof is invalid
        verifier.verifyProof(vkey, abi.encode(commitment), proof);

        (bool success,) = msg.sender.call{value: depositAmount}("");
        require(success, "Transfer failed");
    }

    function root() public view returns (bytes32) {
        return tree.root;
    }
}
