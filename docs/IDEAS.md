
## Ideas

### Tower Defense Part

- pinball table background is lava texture (maybe with animation?)
- the enemies walk from top to bottom on a predefined path with concrete texture
- around the way are predefined places for towers
- Different tower types, which have much advantages when the right types are built near each other

#### Tower Types

- slowdown tower: Slows enemies in radius down
- tesla tower: Damages every enemy in radius constantly
- machine gun tower: Damages one enemy in radius unit dead
- mortar tower: Shoots mortar, hits all enemies in explosion radius, but long reload time
- flame thrower tower: Hits all enemies in a cone, ground burns for seconds, will do extra damage

### Pinball Part

- hitting a tower place fills bar, when bar is filled, tower upgrade is ready
- to upgrade a tower, special fields gets visible/hitable, hit one to select the tower type
- if more upgrades are ready, you can hit the special fields multiple times, the upgrades were build in queue
- every tower is an object with collision, if one gets hit, bar fills, when bar is filled, tower gets upgrade
- (random extra field, when hit, then temporary extra like second ball, bigger ball damage-/radius, etc.)
- If ball hits enemy, it is instant dead
- Hitting the road with the ball, causes a shockwave along the road, which damages the enemies on the road a little

### Base Part

- drill hole with drill, the monsters want to destroy
- every building can be replaced by hitting special field which is only visible when specific building is destroyed
- all towers and factories need power from power plant which only produces power if the drill is running

#### Base Buildings

- Power Plant
- Damage Booster Building

### Game goal

- player win if every resource from drill hole is farmed
- monsters win if there is no of your buildings in base left

### Beginning phase

- Very easy enemy waves, because it must take long to build a new tower
- Only defending by hitting an enemy with the ball

### Upgrade System

- Hit foundation to fill progress bar, if full -> pinball menu tower selection (towers: gun, microwave, tesla, mortar)
- Hit towers to fill progress bar, if full -> pinball menu upgrade selection (possible upgrades: more sight range, damage, faster rotate speed, bigger damgage range)
- every action gives points, collect points to get on a higher level -> unlock more tower types and upgrades

### Extra fields

Hit extra field with ball to get temporal effect.
- Extra ball extra with another ball every 5 seconds over 30 seconds and no ball loose damage

### Mobile

- Extra: Gyroscope can control ball movement

# Scenario

## Scenario 1: Steampunk / Mechanic

### Story

You are defending your Steampunk city against a war lord. He has no army, but access to brain manipulation machines to send waves of enemies, mostly poor people and animals from around, which he slaved by setting the machine on their heads and they are forced to run against your lines. Sometimes he has access to little and rare huge robots aswell. His goal is get through your defense and destroy your food-, ammo- factories and generator to weak you and take over the city. To get away from the war, the civilians flew into the canalisation and tunnels, where they live now while on the topsite the war is raging.

### 3D Models

#### Table

The table is the Steampunk city with homes in the canalisation and tunnels painted on the game field. The surrounding is packed with pipes, steam valves, old bricks, rusty gears and iron.

#### Enemies

Note on modeling: To keep the 3D models simple and easy to create, the enslaved victims are mostly encased in crude, blocky mechanical suits or cages. We don't need to model organic human bodies, just the rusty, industrial metal shells that trap them.

##### Normal: The "Iron Cage"
- **Lore**: A poor civilian trapped inside a heavy, boxy iron container on crude mechanical legs or tracks. The brain chip controls the cage's movement.
- **Model**: A simple rusty box or cylinder with rivets, a glowing red slit for the vision sensor, and 4 blocky legs or simple tank tracks. Very easy primitive shapes.

##### Speeder: The "Monowheel Runner"
- **Lore**: Slaves strapped into unstable, steam-powered single-wheel contraptions to quickly breach the lines.
- **Model**: A large gear or tire-shaped cylinder that rolls forward. It has a small exhaust pipe on the side emitting steam. Fast and requires minimal animation (just rotation).

##### Tank: The "Boiler Golem"
- **Lore**: A massive, walking steam boiler. The warlord crams multiple slaves inside to shovel coal and power the hydraulic systems. 
- **Model**: A large, blocky cylinder (the boiler) with a domed top, a large smokestack, and two huge, boxy arms. Moves slowly and absorbs a lot of damage.

## Scenario 2: 80s Synthwave / Cyberpunk

## Scenario 3: Retro Sci-Fi

## Scenario 4: WWI Trench Fight with green toy soldiers
