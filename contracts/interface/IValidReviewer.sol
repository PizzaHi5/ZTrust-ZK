// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

interface IValidReviewer {
    function rateFinding(uint256 score, uint256 line, bytes32 postStateDigest, bytes calldata seal) external;

    /// @notice Returns the values
    function getScore(uint256 line) external view returns (uint256 score);
}