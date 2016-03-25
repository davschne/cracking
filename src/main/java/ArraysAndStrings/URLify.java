package cracking.arraysAndStrings;

import java.util.Scanner;

// Problem:
// Write a method to replace all spaces in a string with "%20". You may assume that the string has sufficient space at the end to hold the additional characters and that you are given the "true" length of the string. (Note: If implementing in Java, please use a character array so that you can perform this operation in place.)

public class URLify {
  public static char[] urlify(char[] array, length) {
    // find the number of spaces in the array
    int spaces = 0;
    for (int i = 0; i < length; i++) {
      if (array[i] == ' ') spaces++;
    }
    // traverse array in reverse
    for (int i = length - 1; i >= 0; i--) {
      // exit loop if there are no more spaces (remaining chars are already in position)
      if (spaces == 0) break;
      // number of positions by which character must be shifted right (to accommodate the two extra characters needed in replacing each space)
      int offset = 2 * spaces;
      // if char is a space: replace with "%20", decrement spaces counter
      if (array[i] == ' ') {
        array[i + offset - 2] = '%';
        array[i + offset - 1] = '2';
        array[i + offset]     = '0';
        spaces--;
      }
      // otherwise just put in position
      else {
        array[i + offset] = array[i];
      }
    }
    return array;
  }
}
