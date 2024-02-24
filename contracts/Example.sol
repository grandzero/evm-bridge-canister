// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

contract Example {
    bytes public waitingMessage;
    address public sender;
    bytes public incomingMessage;

    function icp_call(bytes memory message) public {
        incomingMessage = message;
    }

    function get_incoming() public view returns (bytes memory) {
        return incomingMessage;
    }

    function send_message(address targetAddress, string memory message) public {
        waitingMessage = abi.encodePacked(targetAddress, message);
    }

    function get_message() public view returns (bytes memory) {
        return waitingMessage;
    }
}
