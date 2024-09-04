# Requirements for consistency from Cap'n Proto 

capnp-angy alerts you when a schema change meets any of the following conditions.
No warning is given in other cases.

| ID    | TARGET    | Summary                                               | 
| ----  | ----      | ----                                                  | 
| C01   | File      | The id is changed.                                    |
| C02   | Struct    | The id is deleted.                                    |
| C03   | Struct    | The type of a field is changed.                       |
| C04   | Struct    | The name of a field is changed.                       |
| C05   | Struct    | The a non-union field becomes union.                  |
| C06   | Struct    | The a union field becomes non-union.                  |
| C07   | Struct    | The default value of a field is changed.              |
| C08   | Struct    | A field is deleted.                                   |
| C09   | Interface | The id is deleted.                                    |
| C10   | Interface | A arg type of a method is changed.                    |
| C11   | Interface | A arg of a method is deleted.                         |
| C12   | Interface | A arg of a method is added.(?)                        |
| C13   | Interface | The default value of a arg of a method is changed.(?) |
| C14   | Interface | The return type of a method is changed.               |
| C15   | Interface | The name of a method is changed.                      |
| C16   | Interface | A method is added at the middle of the Interface.     |
| C17   | Enum      | The size of the enum is changed.                      |
| C18   | Enum      | The name of a value in the enum is changed.           |
| M01   | Struct    | New union field is added .                            |
| M02   | Enum      | New member is added.                                  |
<!--
| C01   | Const     | nop                                                   |
| C01   | Annotation| nop                                                   |
-->

# Normal case for consistency 
The following are cases in which a warning will not be issued:

| ID    | TARGET    | Summary                                                                   |
| ----  | ----      | ----                                                                      |
| N01   | FILE      | There are no changes.                                                     |
| N02   | Struct    | New field is added at the end of the field.                               |
| N03   | Struct    | Change the reference to the external struct from a direct reference to a generics reference. |
| N04   | Struct    | Change the name but the ID is manually set so that it does not change.    |
| N05   | Interface | New method is added at the end of the Interface.                          |
| N06   | Interface | Change the name but the ID is manually set so that it does not change.    |
| N07   | Enum      | Change the name but the ID is manually set so that it does not change.    |
| N08   | Const     | Change the name but the ID is manually set so that it does not change.    |