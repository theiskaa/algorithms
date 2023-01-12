// Simplify Path - https://leetcode.com/problems/simplify-path

/* Problem Statement:
   Given a string path, which is an absolute path (starting with a slash '/')
   to a file or directory in a Unix-style file system, convert it to the simplified canonical path.

   In a Unix-style file system, a period '.' refers to the current directory, a double period '..'
   refers to the directory up a level, and any multiple consecutive slashes (i.e. '//')
   are treated as a single slash '/'. For this problem, any other format of periods such as '...'
   are treated as file/directory names.

   The canonical path should have the following format:

    • The path starts with a single slash '/'.
    • Any two directories are separated by a single slash '/'.
    • The path does not end with a trailing '/'.
    • The path only contains the directories on the path from the root directory to
      the target file or directory (i.e., no period '.' or double period '..')

   Return the simplified canonical path.
*/

/* Examples:
   1.
    Input: path = "/home/"
    Output: "/home"
    Explanation: Note that there is no trailing slash after the last directory name.

   2.
    Input: path = "/../"
    Output: "/"
    Explanation: Going one level up from the root directory is a no-op,
                 as the root level is the highest level you can go.

   3.
    Input: path = "/home//foo/"
    Output: "/home/foo"
    Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
*/

/* Constraints:
   • 1 <= path.length <= 3000
   • path consists of English letters, digits, period '.', slash '/' or '_'.
   • path is a valid absolute Unix path.
*/

package main

import (
	"fmt"
	"strings"
)

func main() {
	tests := map[string]string{
		"/home/":            "/home",
		"/../":              "/",
		"/home//foo":        "/home/foo",
		"/a/./b.//../../c/": "/c",
	}

	fmt.Println("Simplify Path:")
	i := 1
	for input, expected := range tests {
		res := simplifyPath(input)
		if res != expected {
			fmt.Printf(" [X] %d'th test result: %v expected: %v \n", i, res, expected)
		} else {
			fmt.Printf(" [OK] %d'th test result: %v\n", i, res)
		}
		i += 1
	}
}

//  1. Loops through the split of path (by "/"),
//  2. Generates a clearPath array by avoiding not needed char(s)
//     like: "", "."
//  3. Removes the itself and previous element of "..".
//  4. and finally joins into re-build clearPath with "/"
func simplifyPath(path string) string {
	clearPath := []string{}
	for _, ch := range strings.Split(path, "/") {
		if ch == "." || ch == "" {
			continue
		}

		if ch == ".." {
			if len(clearPath) > 0 {
				clearPath = clearPath[:len(clearPath)-1]
			}

			continue
		}

		clearPath = append(clearPath, ch)

	}

	return "/" + strings.Join(clearPath, "/")
}
