// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {SP1Tornado} from "../src/SP1Tornado.sol";
import {SP1VerifierGateway} from "sp1-contracts/src/SP1VerifierGateway.sol";
import {SP1Verifier} from "sp1-contracts/src/v5.0.0/SP1VerifierPlonk.sol";

contract SP1TornadoScript is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();
        // Deploy the verifier gateway
        SP1VerifierGateway gateway = new SP1VerifierGateway(msg.sender);
        // Deploy the verifier
        SP1Verifier verifier = new SP1Verifier();
        // Add the verifier to the gateway
        gateway.addRoute(address(verifier));

        // Get the vkey from the environment
        bytes32 vkey = vm.envBytes32("SP1_PROGRAM_VKEY");
        
        // Deploy the tornado
        new SP1Tornado(1 ether, address(gateway), vkey);

        vm.stopBroadcast();
    }
}
