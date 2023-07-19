# Solar Battery Monitor API
This is the beginning of my solar systems battery monitoring api in which my arduino and nodeMCU will report to. 

I plan to at some point move away from persistent storage and on to some cloud storage, so that i can have some nice extensive history to refer to.

(I may experiment with using my raspberry pi for this)

as of now, my arduino reads the voltages -> sends to nodeMCU -> nodeMCU sends post request to post the battery voltage -> server holds data -> my dashboard requests voltages periodically

This is a real world problem i am solving for myself, as buying hardware from a company like Victron, is not affordable by any means: With this solution i can monitor:
6 batteries (min ~ $650) (deep cycle lead acid - best value at walmart) (non AGM)
1 arduino (~30$) (there are other versions with wifi and more pins and such)
1 nodeMCU (~16$ / 3 pack)
2 wires from walmart (red/black) (~16$)
6 voltage sensors (~10$ / 10pack) (depends where you get them)