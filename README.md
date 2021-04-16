# minecraft-proxy-rs

## Packets still required to be implemented
### Login
 - [ ]  Login CB plugin request
 - [ ]  Login SB plugin response

### Play
#### Serverbound

 - [ ]  Spawn Entity
 - [ ]  Spawn Experience Orb
 - [ ]  Spawn Living Entity
 - [ ]  Spawn Painting
 - [ ]  Spawn Player
 - [ ]  Entity Animation (clientbound)
 - [ ]  Statistics
 - [ ]  Acknowledge Player Digging
 - [ ]  Block Break Animation
 - [ ]  Block Entity Data
 - [ ]  Block Action
 - [ ]  Block Change
 - [ ]  Boss Bar
 - [ ]  Server Difficulty
 - [ ]  Chat Message (clientbound)
 - [ ]  Tab-Complete (clientbound)
 - [ ]  Declare Commands
 - [ ]  Window Confirmation (clientbound)
 - [ ]  Close Window (clientbound)
 - [ ]  Window Items
 - [ ]  Window Property
 - [ ]  Set Slot
 - [ ]  Set Cooldown
 - [ ]  Plugin Message (clientbound)
 - [ ]  Named Sound Effect
 - [ ]  Disconnect (play)
 - [ ]  Entity Status
 - [ ]  Explosion
 - [ ]  Unload Chunk
 - [ ]  Change Game State
 - [ ]  Open Horse Window
 - [ ]  Keep Alive (clientbound)
 - [ ]  Chunk Data
 - [ ]  Effect
 - [ ]  Particle
 - [ ]  Update Light
 - [ ]  Join Game
 - [ ]  Map Data
 - [ ]  Trade List
 - [ ]  Entity Position
 - [ ]  Entity Position and Rotation
 - [ ]  Entity Rotation
 - [ ]  Entity Movement
 - [ ]  Vehicle Move (clientbound)
 - [ ]  Open Book
 - [ ]  Open Window
 - [ ]  Open Sign Editor
 - [ ]  Craft Recipe Response
 - [ ]  Player Abilities (clientbound)
 - [ ]  Combat Event
 - [ ]  Player Info
 - [ ]  Face Player
 - [ ]  Player Position And Look (clientbound)
 - [ ]  Unlock Recipes
 - [ ]  Destroy Entities
 - [ ]  Remove Entity Effect
 - [ ]  Resource Pack Send
 - [ ]  Respawn
 - [ ]  Entity Head Look
 - [ ]  Multi Block Change
 - [ ]  Select Advancement Tab
 - [ ]  World Border
 - [ ]  Camera
 - [ ]  Held Item Change (clientbound)
 - [ ]  Update View Position
 - [ ]  Update View Distance
 - [ ]  Spawn Position
 - [ ]  Display Scoreboard
 - [ ]  Entity Metadata
 - [ ]  Attach Entity
 - [ ]  Entity Velocity
 - [ ]  Entity Equipment
 - [ ]  Set Experience
 - [ ]  Update Health
 - [ ]  Scoreboard Objective
 - [ ]  Set Passengers
 - [ ]  Teams
 - [ ]  Update Score
 - [ ]  Time Update
 - [ ]  Title
 - [ ]  Entity Sound Effect
 - [ ]  Sound Effect
 - [ ]  Stop Sound
 - [ ]  Player List Header And Footer
 - [ ]  NBT Query Response
 - [ ]  Collect Item
 - [ ]  Entity Teleport
 - [ ]  Advancements
 - [ ]  Entity Properties
 - [ ]  Entity Effect
 - [ ]  Declare Recipes
 - [ ]  Tags
#### Serverbound
 - [ ]  Teleport Confirm
 - [ ]  Query Block NBT
 - [ ]  Query Entity NBT
 - [ ]  Set Difficulty
 - [ ]  Chat Message (serverbound)
 - [ ]  Client Status
 - [ ]  Client Settings
 - [ ]  Tab-Complete (serverbound)
 - [ ]  Window Confirmation (serverbound)
 - [ ]  Click Window Button
 - [ ]  Click Window
 - [ ]  Close Window (serverbound)
 - [ ]  Plugin Message (serverbound)
 - [ ]  Edit Book
 - [ ]  Interact Entity
 - [ ]  Generate Structure
 - [ ]  Keep Alive (serverbound)
 - [ ]  Lock Difficulty
 - [ ]  Player Position
 - [ ]  Player Position And Rotation (serverbound)
 - [ ]  Player Rotation
 - [ ]  Player Movement
 - [ ]  Vehicle Move (serverbound)
 - [ ]  Steer Boat
 - [ ]  Pick Item
 - [ ]  Craft Recipe Request
 - [ ]  Player Abilities (serverbound)
 - [ ]  Player Digging
 - [ ]  Entity Action
 - [ ]  Steer Vehicle
 - [ ]  Set Recipe Book State
 - [ ]  Set Displayed Recipe
 - [ ]  Name Item
 - [ ]  Resource Pack Status
 - [ ]  Advancement Tab
 - [ ]  Select Trade
 - [ ]  Set Beacon Effect
 - [ ]  Held Item Change (serverbound)
 - [ ]  Update Command Block
 - [ ]  Update Command Block Minecart
 - [ ]  Creative Inventory Action
 - [ ]  Update Jigsaw Block
 - [ ]  Update Structure Block
 - [ ]  Update Sign
 - [ ]  Animation (serverbound)
 - [ ]  Spectate
 - [ ]  Player Block Placement
 - [ ]  Use Item


## Other TODOS:
 - Making a guide or automated tool to put in the secret logger
 - Making a config file
 - Add tests for *all* values in packets
 - Maybe add tests for cypher.rs
 - Automated loading in of things in server and clientbound