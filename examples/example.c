#include "../target/minecraft_nether_gen_rs.h"
#include <stdio.h>

// gcc -L../target/release -Wl,-rpath=../target/release -o example example.c -lminecraft_nether_gen_rs
// embed the rpath inside the executable, please make sure to be right when doing that
int main(){
    NetherGen* netherGen=create_new_nether(1);
    NetherBiomes biome=get_biome(netherGen,0,0,0);
    if (biome==NetherWastes){
        printf("That's a win\n");
    }
    printf("%llu\n",sizeof (netherGen->seed));
    printf("%llu\n",sizeof (netherGen->_noise));
    printf("%llu\n",sizeof (netherGen->is_3d));
    delete(netherGen);
}
