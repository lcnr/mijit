Register allocation:

 - SP - Stack pointer. Base address for spill slots?
 - R8 - Base address for "persistent" values: those that last longer than one
   call to `mijit_run()`.
 - RC - Temporary.