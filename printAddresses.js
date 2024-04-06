// ES6 imports
import { Keypair } from "@solana/web3.js";
import fs from "fs";
import path from "path";

// Define an asynchronous function to handle reading and processing the keypair files
async function processKeyPairs(start, end) {
  for (let i = start; i <= end; i++) {
    try {
      const filename = path.join("./", `id${i}.json`); // Adjust the path as necessary
      const fileContent = fs.readFileSync(filename, "utf8"); // Read the file content
      const jsonData = JSON.parse(fileContent); // Parse the JSON to get the array

      // Assuming jsonData is directly the secret key or adjust according to your JSON structure
      const account = Keypair.fromSecretKey(new Uint8Array(jsonData));
      const address = account.publicKey.toBase58(); // Get the base58-encoded public address

      console.log(address); // Log the address
    } catch (error) {
      console.error(`Error processing file id${i}.json: ${error}`);
    }
  }
}

// Edit the numbers here for the range of keypairs
processKeyPairs(0, 24);
