//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "hardhat/console.sol";

contract Greeter {
    string private greeting;
    // Declaring state variable  
    int[] private arr; 
        
    // Function to add data 
    // in dynamic array
    function addData(int num) public
    {
        arr.push(num);
    }
        
    // Function to get data of
    // dynamic array
    function getData() public view returns(int[] memory)
    {
        return arr;
    }
        
    // Function to return length 
    // of dynamic array
    function getLength() public view returns (uint)
    {
        return arr.length;
    }

    function greet() public view returns (string memory) {
        return greeting;
    }

    function setGreeting(string memory _greeting) public {
        console.log("Changing greeting from '%s' to '%s'", greeting, _greeting);
        greeting = _greeting;
    }

}
