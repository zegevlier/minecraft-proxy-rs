# How to create the costum jar needed to run this tool

## Step 1: Download MCP Reborn

You can download MCP reborn from [here](https://github.com/Hexeption/MCP-Reborn). Just follow the written instructions on how to decompile minecraft. It's basically just download, import wait a bit and then click a button.

## Step 2: Editing the source code

You now need to edit the source code to print out the secret token whenever it is generated. This token is then saved to the latest log file and can be accessed by this program.
Go to `src > main > java > net > minecraft > network > login > CEncryptionResponsePacket.java`. On (in 1.16) line 23 you will find the function `CEncryptionResponsePacket()`. Somewhere inside that function, add this line below.
```java
System.out.println("Secret Key: " + java.util.Base64.getEncoder().encodeToString(secret.getEncoded()));
```

## Step 3: Building
From this point you should be able to continue with the written guide linked earlier. You need to build the jar file, put it in the right spot and edit some other files.

## Step 4: Done!
You should now have a version profile that you can use whenever you want to run this tool!