# Mongo Aggregation

learning materials: [The MongoDB Aggregation Framework](https://university.mongodb.com/courses/M121/about)

## Note

1. $match

   - A $match stage may contain a $text query operator, but it must be the first stage in a pipeline

   - $match should come early in an aggregation pipeline

   - You cannot use $where with $match

   - $match uses the same query syntax as find

1. $project

   - Once we specify one field to retain, we must specify all fields we want to retain. The \_id field is the only exception to this.

   - Beyond simply removing and retaining fields, $project lets us add new fields.

   - $project can be used as many times as required within an Aggregation pipeline.

   - $project can be used to reassign values to existing field names and to derive entirely new fields.

1. $addFields

1. $geoNear

   - The collection can have one and only one 2d-sphere index

   - If using 2d-sphere, the distance is returned in meters. If using legacy coordinates, the distance is returned in radians.

   - $geoNear must be the first stage in an aggregation pipeline

1. cursor like stages types

   - $limit: { < integer > }

   - $skip: { < integer > }

   - $count: { < name we want the count called > }

   - $sort: { < field we want to sort on >: < integer, direction to sort > }

1. cursor like stages summary

   - $sort, $skip, $limit, and $count are functionally equivalent to the similarly named cursor methods.

   - $sort can take advantage of indexes if used early within a pipeline.

   - By default, $sort will only use up to 100 megabytes of RAM. Setting **allowDiskUse: true** will allow for larger sorts.

1. $sample

   - { $sample { size: < N, how many documents > } }

1. $group

   - { $group: { \_id: < matching/grouping criteria > } }

   - { $group: { \_id: < matching/grouping criteria >, fieldName: < accumulator expression >, ... < as many fieldName: expressions as required > } }

1. $group highlights

   - \_id is where to specify what incoming documents should be grouped on

   - Can use all accumulator expressions within $group

   - $group can be used multiple times within a pipeline

   - It may be necessary to sanitize incoming data

1. accumulator stages with $project

   - available accumulators expressions: $sum, $avg, $max, $min, $stdDevPop, $stdDevSam

   - expressions have no memory between documents

   - may still have to use $reduce or $map for more complex calculations

1. $unwind forms

   - short form: { $unwind: < field path > }

   - long form: { $unwind: { path: < field path >, includeArrayIndex: < string >, preserveNullAndEmptyArrays: < boolean > } }

1. $unwind highlights

   - $unwind only works on array values

   - there are two forms for unwind, short form and long form

   - using unwind on large collections with big documents may lead to performance issues

1. $lookup

   - { $lookup: { from: < collection to join >, localField: < field from the input documents >, foreignField: < field from the documents of the "from" collection >, as: < output array field > } }

   - the **from** collection cannot be sharded

   - the **from** collection must be in the same database

   - the values in **localField** and **foreignField** are matched on equality

   - **as** can be any name, buf if it exists in the working document that field will be overwritten

1. $graphLookup

   - { $graphLookup: { from: < lookup table >, startWith: < expression for value to start from >, connectFromField: < field name to connect from >, connectToField: < field name to connect to >, as: < field name for result array >, maxDepth: < optional field, number of iterations to perform >, depthField: < optional field, field name for number of iterations to reach this node >, restrictSearchWithMatch: < optional field, match condition to apply to lookup > } }
