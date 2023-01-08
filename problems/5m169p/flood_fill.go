// Flood Fill - https://leetcode.com/problems/flood-fill/

/* Problem Statement:
   An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.

   You are also given three integers sr, sc, and color.
   You should perform a flood fill on the image starting from the pixel image[sr][sc].

   To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to
   the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally
   to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with color.

   Return the modified image after performing the flood fill.
*/

/* Examples:
   1.
   Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
   Output: [[2,2,2],[2,2,0],[2,0,1]]
   Explanation: From the center of the image with position (sr, sc) = (1, 1)
                (i.e., the red pixel), all pixels connected by a path of the same color as the starting pixel
                (i.e., the blue pixels) are colored with the new color.

   Note the bottom corner is not colored 2, because it is not 4-directionally connected to the starting pixel.

   2.
   Input: image = [[0,0,0],[0,0,0]], sr = 0, sc = 0, color = 0
   Output: [[0,0,0],[0,0,0]]
   Explanation: The starting pixel is already colored 0, so no changes are made to the image.
*/

package main

import (
	"fmt"
)

func main() {
	tests := []struct {
		image [][]int
		sr    int
		sc    int
		color int
	}{
		{
			image: [][]int{
				{1, 1, 1},
				{1, 1, 0},
				{1, 0, 1},
			},
			sr: 1, sc: 1, color: 2,
		},
		{
			image: [][]int{
				{0, 0, 0},
				{0, 0, 0},
			},
			sr: 0, sc: 0, color: 0,
		},
	}

	var printImage = func(before [][]int, after [][]int) {
		middle := len(before) / 2
		for i, v := range before {
			if i == middle {
				fmt.Printf(" %v  -->  %v\n", v, after[i])
			} else {
				fmt.Printf(" %v       %v\n", v, after[i])
			}
		}
	}

	i := 1
	for _, a := range tests {
		fmt.Printf("%d'th test result: \n", i)
		res := floodFill(a.image, a.sr, a.sc, a.color)
		printImage(a.image, res)
		fmt.Println()
		i += 1
	}
}

//  1. Checks if current pointed image pixel is already painted as [color] or not.
//     if yes, returns current data.
//  2. Calls fill functionality to base info.
func floodFill(image [][]int, sr int, sc int, color int) [][]int {
	if image[sr][sc] == color {
		return image
	}

	fill(image, sr, sc, image[sr][sc], color)

	return image
}

//  1. Cheks for search-row 's and search-column 's limit barier.
//     if it overflows, breaks function.
//  2. Checks for defaultColor match to pointed image pixel color,
//     in case of no match, breaks function.
//  3. Changes image[sr][sc]'s default(Color) to provided color(new).
//  4. Calss fill function to all 4-diagonal rows and columns.
func fill(image [][]int, sr, sc int, defaultColor, color int) {
	isLimit := 0 > sr || 0 > sc || sr >= len(image) || sc >= len(image[0])
	if isLimit || (defaultColor != image[sr][sc]) {
		return
	}

	image[sr][sc] = color

	fill(image, sr-1, sc, defaultColor, color) // top-row, i.e previous row.
	fill(image, sr+1, sc, defaultColor, color) // bottom-row, i.e next row.
	fill(image, sr, sc-1, defaultColor, color) // left-item, i.e previus item in current column.
	fill(image, sr, sc+1, defaultColor, color) // right-item, i.e next item in current column.
}
