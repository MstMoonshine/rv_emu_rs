# Change system topology

- Current component relation:

```
┌─────────┐ 
│System   │ 	Jump instructions are hard to implement
└┬─┬─┬─┬─┬┘ 	under this topology, since the update
 │ │ │ │┌▽─┐	of PC inside stage IF is determined
 │ │ │ ││WB│	by stage EXE. Naively including a
 │ │ │ │└┬─┘	reference of EXE inside IF causes
 │ │ │┌▽─▽┐ 	cyclic reference while weak reference
 │ │ ││MEM│ 	adds to code complexity.
 │ │ │└┬──┘ 
 │ │┌▽─▽┐   
 │ ││EXE│   
 │ │└┬──┘   
 │┌▽─▽┐     
 ││DE │     
 │└┬──┘     
┌▽─▽┐       
│IF │       
└───┘       
```

- Target topology:
```
┌─────────────────────┐		This design dissolves the
│System               │ 	cyclic reference problem.
└┬───┬─┬────┬──┬─┬───┬┘ 	The System is in charge
┌▽──┐│┌▽──┐┌▽─┐│┌▽─┐┌▽─┐	of dispatching pipeline
│EXE│││MEM││IF│││DE││WB│	tasks and feeding data
└───┘│└┬──┘└┬─┘│└┬─┘└┬─┘	to each stage. However, it
┌────▽─▽────▽┐┌▽─▽───▽─┐	does break down the
│Bus         ││Reg File│	intuitive structure of 
└────────────┘└────────┘	the pipeline, which gets
							concealed behind the 
							logic of the System.
```