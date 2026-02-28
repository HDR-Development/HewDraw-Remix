/** 
 * `BufferSettings`
 * An array of buffer settings and their mapped values. Stored in user-mapped controls.
```
[0b00] Standard: 5
[0b01] Low: 3
[0b10] None: 0
[0b11] Standard: 5 // failsafe
``` 
*/
pub const BufferSettings: [u8; 4] = [5, 3, 0, 5];