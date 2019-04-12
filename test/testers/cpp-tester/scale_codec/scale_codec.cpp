// Copyright (c) 2019 Web3 Technologies Foundation

// This file is part of Polkadot RE Test Suite

// Polkadot RE Test Suite is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot RE Tests is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Foobar.  If not, see <https://www.gnu.org/licenses/>.

#include <getopt.h>
#include <iostream>

#include "common/result.hpp"
#include "scale/basic_stream.hpp"
#include "scale/collection.hpp"

using namespace kagome;          // NOLINT
using namespace kagome::common;  // NOLINT
using namespace common::scale;   // NOLINT
using namespace std;

/**
 *  prints usage instructions then exits.
*/
void usage(void)
{
    cerr << "usage: scale_codec encode --input=[INPUT]" << endl;
    exit(1);
}

// /**
//  * @given collection of 80 items of type uint8_t
//  * @when encodeCollection is applied
//  * @then expected result is obtained: header is 2 byte, items are 1 byte each
//  */
// TEST(Scale, encodeCollectionOf80) {
//   // 80 items of value 1
//   const Buffer collection(80, 1);
//   auto out_bytes = Buffer{65, 1};
//   out_bytes.put(collection.toVector());
//   Buffer out;
//   auto &&res = collection::encodeCollection(collection.toVector(), out);
//   ASSERT_TRUE(res);
//   ASSERT_EQ(out.toVector().size(), 82);
//   // clang-format off
//   ASSERT_EQ(out.toVector(), out_bytes.toVector());
//   // clang-format on
// }

/**
 * processes the options related to encode subcommand, encodes
 * the input and pint out the result 
 *
 * @param argv the array containing command line arguments 
 * @param argc number of elements in argv
 * @param subcommand_arg_pos the position wher the subcommand args start
 *
 * @return the index of where the subcommand options ends
 *
 */
int
encode(char* const* argv,  const int argc, const int subcommand_arg_pos)
{
    const struct option long_options[] = {
       {.name = "input", .has_arg = required_argument, .val = 'i' },
     {}
    };
    const char* const short_options = "i:";

    int next_option = 0; //we keep track so we can return the next
                         //subcommand position
    
    do {
      next_option = getopt_long (subcommand_arg_pos, argv, short_options, long_options, NULL);
        
      switch (next_option)
        {
        case 'i': {
                string input_value = optarg;
                cout << input_value;
        }

        case '?':
          /* The user specified an invalid option. */
          /* Print usage information to standard error, and exit with exit
             code one (indicating abnormal termination). */
          usage();
        case -1:
          break;
          /* Done with options.
           */
        default:
            //something went wrong
            cout << "something went wrong with parsing args" << endl; 
            exit(1);
          break;
          }
    }
    while (next_option != -1);

    return optind;
}

int main() {
}
