// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

contract Example {
    bytes public waitingMessage;
    address public sender;
    bytes public incomingMessage;

    function icpCall(bytes memory message) public {
        incomingMessage = message;
        sender = msg.sender;
    }

    function getIncoming(uint256 _index) public view returns (bytes memory) {
        return incomingMessage;
    }

    function sendMessage(address targetAddress, string memory message) public {
        waitingMessage = abi.encodePacked(targetAddress, message);
    }

    function getMessage(uint256 _index) public view returns (bytes memory) {
        return waitingMessage;
    }

    receive() external payable {}

    fallback() external {
        incomingMessage = msg.data;
    }
}
