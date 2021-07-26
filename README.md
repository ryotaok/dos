# dos

dos is a party damage output simulator (up to 2 members for now).

This program may contain the information from the future game, so if you do not want any spoilers, please do not use it.

## Simulation settings

Characters:

- All characters up to version 2.1 are implemented (you can choose a preferable version for simulation).
- Using stats of character level at 90.
- All talent levels are 10.
- Constellations are limited to 0 for now.

When a player presses some button to do a certain action, it does not always mean that the enemy takes damage. There are differences between character's actions and their attack entities (which have a hit box in the game). The player interactions to character's actions are considered as `AttackEvent`, and attack entities that can damage enemies are considered as `Attack`.

In this program, most of `Attack` are crated by characters at the same time as `AttackEvent` is triggered but some are not, especially those have DoT damage over times. Normal and charge attacks are simultaneous.

Weapons:

- Most weapons up to version 2.1 are implemented (you can choose a preferable version for simulation).
- Using stats of weapon level at 90.
- All five star weapons have the refinement rank of 1, while all four star weapons have the refinement rank of 5. Some three star weapons are included.

Artifacts:

- Most artifacts up to version 2.1 are implemented (you can choose a preferable version for simulation).
- All the artifacts have the same stats: 80 ATK%, 80% Crit Rate, 311 flat ATK, 46.6% or 58.3% DMG bonus for the respective character's vision. If the sum of Crit rate exceeds 80%, the excesses are converted to Crit damage.

Damage calculation:

- Most in-game features are implemented (see the Limitations below): enemy resistance, enemy defense, elemental reactions, internal cooldown of elemental application, the gauge unit theory,
- Critical damage is the average value of Crit rate and Crit damage.
- Randomness of passive effects is always 100% (for example, Prototype Archaic has 50% chance to activate its passive but it is always activated when its condition is met).

The simulated field:

- The enemy is a hilichurl at level 90.
- Simulated characters try to take actions every 200 milliseconds and the simulation ends when the simulation timer is at 20 seconds (these values can be changed). If the actions are under cooldown, the characters will do nothing.
- The first character has an attacker role, where the elemental burst, the elemental skill, and the normal or charge attack are taken. While the rest of the characters, if exists, have supporter role, where only the bursts and the skills are taken.
- Characters recharge all the their energy at the begging, and 15 energy are given when the timer is at 5 seconds.

The following image and table show the summary of 15208 combinations of characters, weapons and artifacts simulated using the program. Each row shows the best combination for each character and they are ranked by the total damage at 21 seconds.

## Result of single member simulation

![Result of single member simulation](./images/simulation1_0714.png)

## Limitations

I've only implemented damage related features in the game but they are inaccurate or not tested well. Although the program shows damage outputs at a particular moment, but I cannot guarantee the program predicts the game behavior. The damage outputs do not mean that some characters are superior to others, but some characters are good at dealing damage. Due to the amount of calculation, I haven't checked the results of more than 2 member simulation.

Other implementation limitations:

- The enemy has infinite HP and does not move.
- Characters have infinite stamina.
- Shields are always active.
- When characters use Staff of Homa, their HP is below 50%.
- Amos' Bow travels the maximum distance.
- Elemental resonance is not implemented.

TODOs

- number of hits of Keqing burst should change ICD
- cannot trigger Shatter
- electro charged is not implemented fully
- need tests of elemental reactions: freeze
- Chongyun's Cryo infusion applies to everyone
- Klee skill starts with 2 charges.
- Zhongli A4.
- Beidou dmg bonus?
- Lisa conductive stacks
- Razor sigils?
- Anemo characters should take actions later.
- Prototype Crescent requires enemy’s weak point.
- royal series
- Prototype Amber.
- LithicSpear passive?

## FAQ

- How do you get cooldown of normal or charge attacks?

I recorded my game @60 FPS and save them as 60 FPS videos. I collected the cooldowns from the difference of `stime` and `etime`, where `stime` is when the frame of the first attack shows a damage number, and `etime` is when the fame of the last attack shows one. Although I tried to collect cooldowns accurately, some of them are wrong.

- How characters are switched?

Currently, all party members are on the field.

- I want to use different levels, stats or own data.

All stats are hard coded, so the current program does not support user inputs. I think the program should be able to simulate damages differently.

- Why does Keqing use Skyward Blade?

Within the 20 seconds, she recharged fully and was able to cast her burst again when she used it. This kind of behaviors are found in other characters who recharge fast like Fischl, Albedo and Geo traveler.

- Where is the 2 member simulation result?

Visit [my GitHub Page](https://ryotaok.github.io/dos/overview.html) and navigate to each character.

## Credit

I'd like to thank these sites that I could use their data, information and formulas:

- <https://genshin.honeyhunterworld.com>
- <https://keqingmains.com>
- <https://genshin-impact.fandom.com>

## License

MIT
