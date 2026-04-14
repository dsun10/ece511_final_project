.mode column
.headers on

SELECT 
    ID, 
    TICK,
    fsqId,
    printf('0x%x', startPC) as startPC,
    taken,
    printf('0x%x', controlPC) as controlPC,
    controlType,
    printf('0x%x', fallThruPC) as fallThruPC,
    mispred,
    source,
    printf('0x%x', target) as target
FROM BPTrace 
-- WHERE startPC=0x80000130
LIMIT 100;

SELECT 
    ID, 
    TICK,
    fsqId,
    btbHit,
    printf('0x%x', startPC) as startPC,
    predTaken,
    printf('0x%x', controlPC) as controlPC,
    -- controlType,
    printf('0x%x', predEndPC) as predEndPC,
    -- mispred,
    predSource,
    printf('0x%x', target) as target
FROM PREDTRACE 
-- WHERE startPC=0x80000130
-- WHERE ID>5000
LIMIT 100;



SELECT 
    ID, 
    TICK,
    fsqId,
    ftqId,
    printf('0x%x', startPC) as startPC,
    printf('0x%x', endPC) as endPC,
    taken,
    printf('0x%x', takenPC) as takenPC,
    printf('0x%x', target) as target
FROM FTQTRACE
LIMIT 100;


SELECT 
    ID, 
    TICK,
    printf('0x%x', startPC) as startPC,
    printf('0x%x', branchPC) as branchPC,
    wayIdx,
    mainFound,
    mainCounter,
    mainUseful,
    mainTable,
    mainIndex,
    altFound,
    altCounter,
    altUseful,
    altTable,
    altIndex,
    useAlt,
    predTaken,
    actualTaken,
    allocSuccess
FROM tagemisstrace
-- WHERE startPC=0x80000136 and taken=0 
LIMIT 100;