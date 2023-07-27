# idl_format

The main `IdlFormat` trait defines
- how to deserialize the IDL file
- how to generate rust code from the deserialized struct

Currently each IDL format is completely isolated from each other in its own folder with no common code shared between them. This allows for independent evolution of each format at the cost of (a lot of) duplicated code. Might refactor this in the future.