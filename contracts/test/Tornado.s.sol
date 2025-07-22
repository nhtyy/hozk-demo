// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {SP1Tornado} from "../src/SP1Tornado.sol";

contract SP1TornadoTest is Test {
    SP1Tornado public tornado;

    function setUp() public {
        tornado = new SP1Tornado(1 ether, address(this), bytes32(0));
    }

    function testDepoistAndLogRoot() public {
        vm.deal(address(this), 6 ether);

        bytes32 one = bytes32(uint256(1));
        bytes32 two = bytes32(uint256(2));
        bytes32 three = bytes32(uint256(3));
        bytes32 four = bytes32(uint256(4));
        bytes32 five = bytes32(uint256(5));

        tornado.deposit{value: 1 ether}(one);
        console.logBytes32(tornado.root());

        tornado.deposit{value: 1 ether}(two);
        console.logBytes32(tornado.root());

        tornado.deposit{value: 1 ether}(three);
        console.logBytes32(tornado.root());

        tornado.deposit{value: 1 ether}(four);
        console.logBytes32(tornado.root());

        tornado.deposit{value: 1 ether}(five);
        console.logBytes32(tornado.root());
    }
}
