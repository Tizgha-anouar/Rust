//memory intermediate
//memory is stored using binary
//bits => 0 or 1
//computer is optimized for bytes
//1 byte = 8 bits
//memory is stream of bits
//all data has an address
// offsets
//ites can be located at an address using an >> offset
//offset begin at 0
//repressent the number of bytes away from the original address
//addresses and offsets
// ▮ = 1 byte  
//          0 1 2 3     
//        0 ▮ ▮ ▮ ▮ 
//address 4 ▮ ▮ ▮ ▮  if we want to access data 
//        8 ▮ ▮ ▮ ▮     specify address and offset
//       12 ▮ ▮ ▮ ▮     ex: address 4 - offset 2 
//                           data[2]
//                           address = data
//                          offset  = 2
//                           intead of usinf address and offset => using index