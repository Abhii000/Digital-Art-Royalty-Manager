# Digital Art Royalty Manager

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

---

## Project Title:
**Digital Art Royalty Manager**

---

## Project Description:
The **Digital Art Royalty Manager** is a decentralized smart contract platform built on Soroban, designed to help digital artists manage and track royalties for their artwork. It allows creators to define royalties on their artwork and ensures that whenever the artwork is transferred or sold, the creator automatically receives a specified percentage of the proceeds. The platform also provides transparency to both artists and buyers, allowing for easy tracking of royalty payments and transfers.

---

## Project Vision:
To empower digital artists by providing a transparent and automated way to manage royalties for their artwork, ensuring they are fairly compensated every time their artwork is transferred, sold, or resold. This platform aims to build trust between creators and buyers while eliminating intermediaries, giving more control to the artist.

---

## Key Features:
- **Royalty Tracking**: Automatically track and distribute royalties to the artist every time the artwork is sold or transferred.
- **Ownership Verification**: Use blockchain to verify the ownership and authenticity of digital artwork.
- **Transparency**: Provide full visibility into royalty payments and the history of artwork transfers.
- **Fair Compensation**: Ensure that artists are compensated according to the royalty percentage set by them.
- **Smart Contract Automation**: Automate royalty distribution via smart contracts to avoid manual errors or delays.

---

## Contract Details:
#### Contract Address: CBVSWZ3ML4ORIIEA6TUI5NE2W6DMQVKBMZULQQFTGRGXW3OTKBH2NLAB
#### Overview:
The **Digital Art Royalty Manager** contract allows digital artists to set up a royalty system for their artworks. The contract will handle the collection of royalties when the artwork is sold or transferred. It supports tracking multiple artworks, each with its own royalty percentage and a history of transactions.

#### Functions:

- **create_artwork**: This function allows the artist to create a new artwork entry with a title, creator's address, and the royalty percentage they want to receive for each sale of the artwork.
- **transfer_artwork**: When the artwork is sold, the buyer can call this function to pay the creator their royalty percentage from the sale price.
- **get_artwork**: This function allows anyone to retrieve details of a specific artwork by its `artwork_id`.
