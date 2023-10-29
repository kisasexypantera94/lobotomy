# Design of L2Book builder from L3 (or L3Delta) events

## Option 1
`L3 => L2Delta => L2BookBuilder::apply_l2_deltas`
1. L3 events are probably going to be in the cache (aren't they?) so we can sort them first for a better cache hit rate.
2. Now we can aggregate them into an array of L2Delta events `{px, amt_delta}` (amt_delta is not necessarily a number, it could be a function).
3. Apply L2Delta events

## Option 2
`L3 => L2BookBuilder::apply_l3_deltas`

Store all the orders into vector `orders`, using EntryID as the key. On some exchanges, EntryIDs are incremental, so using a vector is a must. Applying l3 delta and deducing l2 delta should be fast and cache-friendly because new orders are grouped together.

???
