:warning: **Work in progress | At the moment broken**

# Pinball Defense

<p align="center">
  <img src="assets/demo-animation.gif"/>
</p>

A tower denfense game, but you can only interact with the world by hitting things with the pinball
Story: You want to steal resources on lava planet, but native monsters come in waves to stop you.

Ideas:

Tower Defense Part:

- pinball table background is lava texture (maybe with animation?)
- the enemies walk from top to bottom on a predefined path with concrete texture
- around the way are predefined places for towers
- Different tower types, which have much advantages when the right types are built near each other

Tower Types:

- slowdown tower: Slows enemies in radius down
- tesla tower: Damages every enemy in radius constantly
- machine gun tower: Damages one enemy in radius unit dead

Pinball Part:

- hitting a tower place fills bar, when bar is filled, tower upgrade is ready
- to upgrade a tower, special fields gets visible/hitable, hit one to select the tower type
- if more upgrades are ready, you can hit the special fields multiple times, the upgrades were build in queue
- every tower is an object with collision, if one gets hit, bar fills, when bar is filled, tower gets upgrade
- (random extra field, when hit, then temporary extra like second ball, bigger ball damage-/radius, etc.)
- If ball hits enemy, it is instant dead
- Hitting the road with the ball, causes a shockwave along the road, which damages the enemies on the road a little

Base Part:

- drill hole with drill, the monsters want to destroy
- every building can be replaced by hitting special field which is only visible when specific building is destroyed
- all towers and factories need power from power plant which only produces power if the drill is running

Base Buildings:

- Power Plant
- Damage Booster Building

Game goal:

- player win if every resource from drill hole is farmed
- monsters win if there is no of your buildings in base left

Beginning phase:

- Very easy enemy waves, because it must take long to build a new tower
- Only defending by hitting an enemy with the ball

Upgrade System:

- Hit foundation to fill progress bar, if full -> pinball menu tower selection (towers: gun, microwave, tesla, mortar)
- Hit towers to fill progress bar, if full -> pinball menu upgrade selection (possible upgrades: more sight range, damage, faster rotate speed, bigger damgage range)
- every action gives points, collect points to get on a higher level -> unlock more tower types and upgrades

Mobile:

- Extra: Gyroscope can control ball movement
