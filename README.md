# Genshin

Genshin is a party damage simulator (up to 2 members for now).

This program may contain the information from the future game, so if you do not want any spoilers, please do not use it.

## Simulation settings

Characters:

- All characters up to version 2.0 are implemented (you can choose a preferable version for simulation).
- Using stats of character level at 90.
- All talent levels are 10.
- Constellations are limited to 0 for now.

Weapons:

- Most weapons up to version 2.0 are implemented (you can choose a preferable version for simulation).
- Using stats of weapon level at 90.
- All five star weapons have the refinement rank of 1, while all four star weapons have the refinement rank of 5. Some three star weapons are included.

Artifacts:

- Most artifacts up to version 2.0 are implemented (you can choose a preferable version for simulation).
- All the artifacts have the same stats: 80 ATK%, 80% Crit Rate, 311 flat ATK, 46.6% or 58.3% DMG bonus for the respective character's vision. If the sum of Crit rate exceeds 80%, the excesses are converted to Crit damage.

Damage calculation:

- Most in-game features are implemented (see the Limitations below): enemy resistance, enemy defense, elemental reactions, internal cooldown of elemental application, the gauge unit theory,
- Critical damage is an expected value of Crit rate and Crit damage.
- Randomness of passive effects is always 100% (for example, Prototype Archaic has 50% chance to activate its passive but it is always activated when its condition is met).

Simulated field:

- The enemy is a hilichurl at level 90.
- Simulated characters try to take actions every 200 milliseconds and the simulation ends when the simulation timer is at 20 seconds (these values can be changed). If the actions are under cooldown, the characters will do nothing.
- The first character has an attacker role, where the elemental burst, the elemental skill, and the normal or charge attack are taken. While the rest of the characters, if exists, have supporter role, where only the bursts and the skills are taken.
- Characters recharge all the their energy at the begging, and 15 energy are given when the timer is at 5 seconds.

## Result of single member simulation

<table border="1" class="dataframe"> <thead> <tr style="text-align: right;"> <th>character1</th> <th>weapon1</th> <th>artifact1</th> <th>total</th> <th>1</th> <th>2</th> <th>3</th> <th>4</th> <th>5</th> <th>6</th> <th>7</th> <th>8</th> <th>9</th> <th>10</th> <th>11</th> <th>12</th> <th>13</th> <th>14</th> <th>15</th> <th>16</th> <th>17</th> <th>18</th> <th>19</th> <th>20</th> <th>21</th> </tr> </thead> <tbody> <tr> <td>Eula</td> <td>Song of Broken Pines</td> <td>Reminiscence of Shime</td> <td>566555</td> <td>19135</td> <td>31744</td> <td>38742</td> <td>56360</td> <td>67992</td> <td>117494</td> <td>143124</td> <td>157061</td> <td>333626</td> <td>351531</td> <td>377161</td> <td>391098</td> <td>405980</td> <td>420971</td> <td>442428</td> <td>454096</td> <td>467845</td> <td>513578</td> <td>537257</td> <td>550133</td> <td>566555</td> </tr> <tr> <td>Hu Tao</td> <td>Staff of Homa</td> <td>Retracing Bolide</td> <td>449531</td> <td>23332</td> <td>49365</td> <td>75398</td> <td>101432</td> <td>127465</td> <td>161017</td> <td>187050</td> <td>213084</td> <td>239117</td> <td>272669</td> <td>287742</td> <td>297351</td> <td>306961</td> <td>316570</td> <td>326179</td> <td>335789</td> <td>345398</td> <td>371432</td> <td>397465</td> <td>423498</td> <td>449531</td> </tr> <tr> <td>Yoimiya</td> <td>Thundering Pulse</td> <td>Retracing Bolide</td> <td>417233</td> <td>7604</td> <td>25935</td> <td>43095</td> <td>74534</td> <td>92429</td> <td>125189</td> <td>153854</td> <td>176063</td> <td>204728</td> <td>226937</td> <td>255602</td> <td>275067</td> <td>294131</td> <td>303388</td> <td>322452</td> <td>331710</td> <td>350774</td> <td>360031</td> <td>379095</td> <td>392305</td> <td>417233</td> </tr> <tr> <td>Xiao</td> <td>Staff of Homa</td> <td>Reminiscence of Shime</td> <td>409059</td> <td>0</td> <td>45147</td> <td>71832</td> <td>71832</td> <td>99058</td> <td>126284</td> <td>154051</td> <td>154051</td> <td>181818</td> <td>209585</td> <td>209585</td> <td>258180</td> <td>291894</td> <td>326148</td> <td>326148</td> <td>360403</td> <td>376621</td> <td>376621</td> <td>392840</td> <td>409059</td> <td>409059</td> </tr> <tr> <td>Ayaka</td> <td>Mistsplitter's Reforged</td> <td>Gladiator's Finale</td> <td>380509</td> <td>7532</td> <td>41155</td> <td>66193</td> <td>107759</td> <td>138156</td> <td>175849</td> <td>204761</td> <td>227919</td> <td>234885</td> <td>251141</td> <td>261644</td> <td>284159</td> <td>299640</td> <td>309642</td> <td>316276</td> <td>331756</td> <td>341758</td> <td>348393</td> <td>363873</td> <td>368717</td> <td>380509</td> </tr> <tr> <td>Ganyu</td> <td>Amos' Bow</td> <td>Reminiscence of Shime</td> <td>359306</td> <td>0</td> <td>45820</td> <td>49766</td> <td>53712</td> <td>88654</td> <td>92600</td> <td>123596</td> <td>127542</td> <td>131488</td> <td>166430</td> <td>170376</td> <td>231068</td> <td>231068</td> <td>235014</td> <td>277760</td> <td>281706</td> <td>320506</td> <td>320506</td> <td>320506</td> <td>359306</td> <td>359306</td> </tr> <tr> <td>Diluc</td> <td>The Unforged</td> <td>Gladiator's Finale</td> <td>316881</td> <td>13388</td> <td>46639</td> <td>59183</td> <td>86114</td> <td>94805</td> <td>103295</td> <td>125851</td> <td>134542</td> <td>143033</td> <td>152607</td> <td>174280</td> <td>206160</td> <td>215734</td> <td>228716</td> <td>245897</td> <td>255471</td> <td>268453</td> <td>277144</td> <td>295209</td> <td>308191</td> <td>316881</td> </tr> <tr> <td>Tartaglia</td> <td>Thundering Pulse</td> <td>Pale Flame</td> <td>315058</td> <td>35498</td> <td>49894</td> <td>58185</td> <td>74441</td> <td>85753</td> <td>99184</td> <td>113269</td> <td>125814</td> <td>136767</td> <td>149386</td> <td>161314</td> <td>172145</td> <td>188977</td> <td>194321</td> <td>210039</td> <td>221801</td> <td>267013</td> <td>284771</td> <td>290511</td> <td>303130</td> <td>315058</td> </tr> <tr> <td>Razor</td> <td>Song of Broken Pines</td> <td>Reminiscence of Shime</td> <td>306050</td> <td>6741</td> <td>21856</td> <td>34950</td> <td>50977</td> <td>58484</td> <td>76081</td> <td>101589</td> <td>121780</td> <td>131619</td> <td>148641</td> <td>168242</td> <td>186097</td> <td>207381</td> <td>222720</td> <td>233149</td> <td>253103</td> <td>267593</td> <td>285253</td> <td>292331</td> <td>298427</td> <td>306050</td> </tr> <tr> <td>Klee</td> <td>Lost Prayer to the Sacred Winds</td> <td>2 GF 2 Shime</td> <td>283184</td> <td>0</td> <td>24522</td> <td>48903</td> <td>72067</td> <td>89923</td> <td>114098</td> <td>128038</td> <td>151137</td> <td>168501</td> <td>179853</td> <td>189881</td> <td>196582</td> <td>206610</td> <td>219174</td> <td>224011</td> <td>236575</td> <td>248382</td> <td>253976</td> <td>265783</td> <td>271376</td> <td>283184</td> </tr> <tr> <td>Keqing</td> <td>Skyward Blade</td> <td>Gladiator's Finale</td> <td>278959</td> <td>25079</td> <td>43140</td> <td>53112</td> <td>64770</td> <td>74742</td> <td>86805</td> <td>98767</td> <td>104422</td> <td>128302</td> <td>134664</td> <td>147449</td> <td>153812</td> <td>166597</td> <td>198864</td> <td>209809</td> <td>215464</td> <td>240574</td> <td>247761</td> <td>262204</td> <td>268986</td> <td>278959</td> </tr> <tr> <td>Kazuha</td> <td>Primordial Jade Cutter</td> <td>Reminiscence of Shime</td> <td>278520</td> <td>15130</td> <td>46187</td> <td>53161</td> <td>69436</td> <td>72512</td> <td>86406</td> <td>95762</td> <td>108850</td> <td>112731</td> <td>129006</td> <td>165449</td> <td>174626</td> <td>186937</td> <td>190984</td> <td>200161</td> <td>212472</td> <td>216519</td> <td>225696</td> <td>238007</td> <td>274449</td> <td>278520</td> </tr> <tr> <td>Xingqiu</td> <td>Primordial Jade Cutter</td> <td>Gladiator's Finale</td> <td>269628</td> <td>0</td> <td>25611</td> <td>42555</td> <td>49819</td> <td>63456</td> <td>80400</td> <td>98651</td> <td>104012</td> <td>118246</td> <td>136496</td> <td>152843</td> <td>156091</td> <td>174342</td> <td>190689</td> <td>193937</td> <td>212187</td> <td>228534</td> <td>242768</td> <td>250033</td> <td>263670</td> <td>269628</td> </tr> <tr> <td>Albedo</td> <td>Primordial Jade Cutter</td> <td>Seal of Insulation</td> <td>265983</td> <td>42121</td> <td>51582</td> <td>54833</td> <td>68569</td> <td>69988</td> <td>90700</td> <td>95018</td> <td>107273</td> <td>109106</td> <td>130885</td> <td>132304</td> <td>144973</td> <td>149291</td> <td>168171</td> <td>171422</td> <td>185158</td> <td>186577</td> <td>249410</td> <td>251331</td> <td>264564</td> <td>265983</td> </tr> <tr> <td>Venti</td> <td>Thundering Pulse</td> <td>Pale Flame</td> <td>265533</td> <td>0</td> <td>19390</td> <td>28827</td> <td>41903</td> <td>50854</td> <td>64644</td> <td>73289</td> <td>98425</td> <td>109784</td> <td>123931</td> <td>136032</td> <td>146353</td> <td>154066</td> <td>176748</td> <td>186980</td> <td>195338</td> <td>201728</td> <td>216347</td> <td>227705</td> <td>249521</td> <td>265533</td> </tr> <tr> <td>Jean</td> <td>Primordial Jade Cutter</td> <td>Pale Flame</td> <td>264140</td> <td>21892</td> <td>42166</td> <td>52654</td> <td>65487</td> <td>75975</td> <td>84625</td> <td>94886</td> <td>116824</td> <td>124647</td> <td>135382</td> <td>143205</td> <td>153940</td> <td>161764</td> <td>185786</td> <td>196436</td> <td>204345</td> <td>214994</td> <td>222903</td> <td>233553</td> <td>254578</td> <td>264140</td> </tr> <tr> <td>Bennett</td> <td>Primordial Jade Cutter</td> <td>Pale Flame</td> <td>264064</td> <td>15739</td> <td>28700</td> <td>35020</td> <td>50101</td> <td>59396</td> <td>77484</td> <td>82248</td> <td>101860</td> <td>105272</td> <td>124712</td> <td>130451</td> <td>147736</td> <td>152095</td> <td>172915</td> <td>176471</td> <td>194559</td> <td>215860</td> <td>235473</td> <td>238884</td> <td>253561</td> <td>264064</td> </tr> <tr> <td>Yanfei</td> <td>Lost Prayer to the Sacred Winds</td> <td>Reminiscence of Shime</td> <td>263409</td> <td>10552</td> <td>37867</td> <td>45280</td> <td>51671</td> <td>56069</td> <td>77359</td> <td>85480</td> <td>93225</td> <td>96752</td> <td>104829</td> <td>138272</td> <td>148425</td> <td>157313</td> <td>163533</td> <td>191186</td> <td>202179</td> <td>206445</td> <td>217438</td> <td>227923</td> <td>259143</td> <td>263409</td> </tr> <tr> <td>Rosaria</td> <td>Staff of Homa</td> <td>Pale Flame</td> <td>243306</td> <td>14615</td> <td>30335</td> <td>37299</td> <td>55143</td> <td>58313</td> <td>73797</td> <td>83121</td> <td>108500</td> <td>117116</td> <td>137433</td> <td>141355</td> <td>149971</td> <td>161506</td> <td>178366</td> <td>186981</td> <td>192189</td> <td>202438</td> <td>211054</td> <td>216262</td> <td>235526</td> <td>243306</td> </tr> <tr> <td>Beidou</td> <td>Song of Broken Pines</td> <td>Pale Flame</td> <td>242582</td> <td>5482</td> <td>23672</td> <td>31606</td> <td>44934</td> <td>55706</td> <td>68047</td> <td>77237</td> <td>88010</td> <td>117626</td> <td>128228</td> <td>145765</td> <td>159740</td> <td>165292</td> <td>182830</td> <td>191756</td> <td>197308</td> <td>224161</td> <td>228296</td> <td>232416</td> <td>237552</td> <td>242582</td> </tr> <tr> <td>Traveler (Anemo)</td> <td>Primordial Jade Cutter</td> <td>Gladiator's Finale</td> <td>240792</td> <td>0</td> <td>22590</td> <td>34180</td> <td>50012</td> <td>58604</td> <td>72551</td> <td>82357</td> <td>93455</td> <td>104895</td> <td>136039</td> <td>149494</td> <td>158578</td> <td>168384</td> <td>179482</td> <td>184896</td> <td>194702</td> <td>199775</td> <td>222856</td> <td>230306</td> <td>235228</td> <td>240792</td> </tr> <tr> <td>Fischl</td> <td>Thundering Pulse</td> <td>Pale Flame</td> <td>239304</td> <td>33049</td> <td>42362</td> <td>55244</td> <td>70731</td> <td>74287</td> <td>87771</td> <td>103258</td> <td>115612</td> <td>125325</td> <td>140812</td> <td>153167</td> <td>157457</td> <td>172064</td> <td>183801</td> <td>193120</td> <td>207726</td> <td>214435</td> <td>218726</td> <td>228304</td> <td>231560</td> <td>239304</td> </tr> <tr> <td>Mona</td> <td>Lost Prayer to the Sacred Winds</td> <td>2 GF 2 Shime</td> <td>236532</td> <td>38948</td> <td>57119</td> <td>66007</td> <td>76431</td> <td>84686</td> <td>97144</td> <td>105319</td> <td>111792</td> <td>121412</td> <td>126709</td> <td>136681</td> <td>142495</td> <td>151939</td> <td>169445</td> <td>177353</td> <td>190126</td> <td>196177</td> <td>208291</td> <td>218248</td> <td>226156</td> <td>236532</td> </tr> <tr> <td>Ningguang</td> <td>Lost Prayer to the Sacred Winds</td> <td>2 GF 2 Shime</td> <td>229906</td> <td>26320</td> <td>57285</td> <td>57285</td> <td>73365</td> <td>73365</td> <td>90121</td> <td>90121</td> <td>106877</td> <td>106877</td> <td>124307</td> <td>124307</td> <td>140725</td> <td>140725</td> <td>175589</td> <td>175589</td> <td>193695</td> <td>193695</td> <td>211800</td> <td>211800</td> <td>229906</td> <td>229906</td> </tr> <tr> <td>Xiangling</td> <td>Staff of Homa</td> <td>Pale Flame</td> <td>227287</td> <td>13712</td> <td>15863</td> <td>26805</td> <td>46794</td> <td>55780</td> <td>74355</td> <td>82436</td> <td>100373</td> <td>109997</td> <td>130677</td> <td>142335</td> <td>151959</td> <td>160040</td> <td>169026</td> <td>175002</td> <td>189362</td> <td>192029</td> <td>204284</td> <td>212365</td> <td>221311</td> <td>227287</td> </tr> <tr> <td>Traveler (Geo)</td> <td>Primordial Jade Cutter</td> <td>Seal of Insulation</td> <td>217612</td> <td>38184</td> <td>55032</td> <td>58899</td> <td>65861</td> <td>67645</td> <td>73296</td> <td>80259</td> <td>95321</td> <td>100973</td> <td>106151</td> <td>109719</td> <td>113586</td> <td>120548</td> <td>135611</td> <td>141262</td> <td>143601</td> <td>150008</td> <td>153875</td> <td>157999</td> <td>174117</td> <td>217612</td> </tr> <tr> <td>Lisa</td> <td>Lost Prayer to the Sacred Winds</td> <td>2 GF 2 Elemental</td> <td>208897</td> <td>0</td> <td>23273</td> <td>30083</td> <td>39789</td> <td>48291</td> <td>59270</td> <td>67493</td> <td>76396</td> <td>87594</td> <td>93813</td> <td>105491</td> <td>113916</td> <td>123270</td> <td>133159</td> <td>144239</td> <td>151827</td> <td>163796</td> <td>190918</td> <td>196576</td> <td>200947</td> <td>208897</td> </tr> <tr> <td>Kaeya</td> <td>Primordial Jade Cutter</td> <td>Pale Flame</td> <td>205997</td> <td>0</td> <td>12422</td> <td>26051</td> <td>37712</td> <td>44298</td> <td>57928</td> <td>69588</td> <td>87115</td> <td>98246</td> <td>116035</td> <td>123386</td> <td>130380</td> <td>139895</td> <td>153285</td> <td>160279</td> <td>164517</td> <td>173008</td> <td>180002</td> <td>184240</td> <td>199692</td> <td>205997</td> </tr> <tr> <td>Xinyan</td> <td>Song of Broken Pines</td> <td>Pale Flame</td> <td>200832</td> <td>0</td> <td>30597</td> <td>36444</td> <td>55027</td> <td>60388</td> <td>67261</td> <td>82062</td> <td>89115</td> <td>94296</td> <td>102675</td> <td>116151</td> <td>123023</td> <td>129710</td> <td>139516</td> <td>150058</td> <td>156745</td> <td>164299</td> <td>169290</td> <td>180337</td> <td>195841</td> <td>200832</td> </tr> <tr> <td>Sayu</td> <td>Song of Broken Pines</td> <td>Pale Flame</td> <td>195406</td> <td>4601</td> <td>19598</td> <td>22789</td> <td>33557</td> <td>37059</td> <td>47040</td> <td>55299</td> <td>75015</td> <td>85890</td> <td>96723</td> <td>101830</td> <td>114236</td> <td>123538</td> <td>142517</td> <td>150960</td> <td>156066</td> <td>165328</td> <td>169208</td> <td>173929</td> <td>191479</td> <td>195406</td> </tr> <tr> <td>Noelle</td> <td>Song of Broken Pines</td> <td>Gladiator's Finale</td> <td>193437</td> <td>8607</td> <td>16573</td> <td>21928</td> <td>37011</td> <td>43139</td> <td>55502</td> <td>70416</td> <td>76098</td> <td>91566</td> <td>103376</td> <td>110057</td> <td>124971</td> <td>137334</td> <td>146121</td> <td>157931</td> <td>170864</td> <td>174603</td> <td>178070</td> <td>182146</td> <td>189698</td> <td>193437</td> </tr> <tr> <td>Sucrose</td> <td>Lost Prayer to the Sacred Winds</td> <td>2 GF 2 Elemental</td> <td>176172</td> <td>0</td> <td>16061</td> <td>21100</td> <td>38285</td> <td>43033</td> <td>61806</td> <td>68036</td> <td>84646</td> <td>91869</td> <td>95917</td> <td>103486</td> <td>107849</td> <td>114998</td> <td>120662</td> <td>127386</td> <td>132723</td> <td>154245</td> <td>158447</td> <td>166306</td> <td>170835</td> <td>176172</td> </tr> <tr> <td>Chongyun</td> <td>Serpent Spine R5</td> <td>Gladiator's Finale</td> <td>168784</td> <td>20182</td> <td>32754</td> <td>36761</td> <td>48286</td> <td>51649</td> <td>54890</td> <td>64213</td> <td>67808</td> <td>71050</td> <td>75441</td> <td>84801</td> <td>98436</td> <td>102827</td> <td>108696</td> <td>116415</td> <td>121072</td> <td>140091</td> <td>146059</td> <td>158289</td> <td>164493</td> <td>168784</td> </tr> <tr> <td>Amber</td> <td>Skyward Harp</td> <td>Seal of Insulation</td> <td>161047</td> <td>33335</td> <td>48659</td> <td>48659</td> <td>55338</td> <td>55338</td> <td>64569</td> <td>64569</td> <td>71249</td> <td>71249</td> <td>80480</td> <td>80480</td> <td>87160</td> <td>87160</td> <td>96391</td> <td>96391</td> <td>103070</td> <td>145137</td> <td>154368</td> <td>154368</td> <td>161047</td> <td>161047</td> </tr> <tr> <td>Zhongli</td> <td>Staff of Homa</td> <td>Pale Flame</td> <td>137837</td> <td>32262</td> <td>34937</td> <td>38580</td> <td>45815</td> <td>49344</td> <td>56664</td> <td>61944</td> <td>70465</td> <td>72457</td> <td>80283</td> <td>83064</td> <td>91585</td> <td>95596</td> <td>101404</td> <td>106970</td> <td>114699</td> <td>116716</td> <td>125306</td> <td>128090</td> <td>135819</td> <td>137837</td> </tr> <tr> <td>Diona</td> <td>Thundering Pulse</td> <td>2 bc 2 pf</td> <td>134103</td> <td>4334</td> <td>18084</td> <td>23327</td> <td>32590</td> <td>34983</td> <td>43078</td> <td>49489</td> <td>56957</td> <td>59977</td> <td>69241</td> <td>73857</td> <td>79474</td> <td>85346</td> <td>92425</td> <td>95191</td> <td>101062</td> <td>114812</td> <td>120055</td> <td>126466</td> <td>128860</td> <td>134103</td> </tr> <tr> <td>Qiqi</td> <td>Primordial Jade Cutter</td> <td>Pale Flame</td> <td>127374</td> <td>15068</td> <td>22272</td> <td>26751</td> <td>32523</td> <td>37721</td> <td>43285</td> <td>50457</td> <td>58135</td> <td>61218</td> <td>68390</td> <td>76068</td> <td>79152</td> <td>86323</td> <td>94001</td> <td>97085</td> <td>104257</td> <td>109146</td> <td>112229</td> <td>119401</td> <td>121810</td> <td>127374</td> </tr> <tr> <td>Barbara</td> <td>Skyward Atlas</td> <td>Retracing Bolide</td> <td>120744</td> <td>0</td> <td>3643</td> <td>8423</td> <td>16328</td> <td>20949</td> <td>29716</td> <td>35398</td> <td>42242</td> <td>48786</td> <td>54652</td> <td>61312</td> <td>67337</td> <td>73722</td> <td>80725</td> <td>86407</td> <td>93251</td> <td>99795</td> <td>105662</td> <td>112321</td> <td>116124</td> <td>120744</td> </tr> </tbody> </table>

## Limitations

I've only implemented damage related features in the game but they are inaccurate or not tested well. Although the program shows damage outputs at a particular moment, but I cannot guarantee the program predicts the game behavior. The damage outputs do not mean that some characters are superior to others, but some characters are good at dealing damage. Due to the amount of calculation, I haven't checked the results of more than 2 member simulation.

Other implementation limitations:

- The enemy has infinite HP and does not move.
- Characters have infinite stamina.
- Shields are always active.
- When characters use Staff of Homa, their HP is below 50%.
- Amos' Bow travels the maximum distance.

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

Within the 20 seconds, she recharged fully and was able to cast her burst again when she used it. This kind of behaviors are found in other characters who recharge fast like Fischl, Albedo and Geo traveler. But the weapon may be bugged.

- Where is the 2 member simulation result?

My computer did not finish the calculation in time. Please wait.

- Which character do you like?

Mona :)

## Credit

I'd like to thank these sites that I could use their data, information and formulas:

- <https://genshin.honeyhunterworld.com/>
- <https://keqingmains.com/>
- <https://genshin-impact.fandom.com/wiki/Genshin_Impact_Wiki>

## License

MIT