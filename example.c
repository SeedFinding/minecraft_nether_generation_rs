#include "target/release/minecraft_nether_gen_rs.h"
#include <stdio.h>

// gcc -

int main(){
    NetherGen *netherGen=create_new_nether(1);
    printf("%lld\n",netherGen->seed);
}
