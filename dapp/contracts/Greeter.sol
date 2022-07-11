//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "hardhat/console.sol";

contract Greeter {
    string private greeting;
    string [] private strings;

    function addString (string memory str) public {
        strings.push (str);
    }

    function getStrings () public view returns (string [] memory) {
        return strings;
    }

    function getLength() public view returns (uint)
    {
        return strings.length;
    }

    function greet() public view returns (string memory) {
        return greeting;
    }

    function setGreeting(string memory _greeting) public {
        console.log("Changing greeting from '%s' to '%s'", greeting, _greeting);
        greeting = _greeting;
    }

}
