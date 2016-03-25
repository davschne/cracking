package cracking.arraysAndStrings;

import java.util.Scanner;

// Problem:
// Given a string, write a function to check if it is a permutation of a palindrome. A palindrome is a word or phrase that is the same forwards and backwards. A permutation is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.

public class PalindromePermutation {
  public static boolean palindromepermutation(String input) {
    // In a palindrome, every character but the middle characters appears an even number of times. The middle character may appear either an even or an odd number of times, depending on whether the total number of characters is even or odd.

    // use an array to store the number of each type of character in the string
    int[] counts = new int[256];
    // initialize with 0 values
    for (int i = 0; i < counts.length; i++) {
      counts[i] = 0;
    }
    // compute the number of occurrences of each character
    for (int i = 0; i < input.length(); i++) {
      char c = input.charAt(i);
      // disregard spaces
      if (c != ' ') counts[c]++;
    }
    // make allowance for one of the characters to appear an odd number of times
    boolean oddAllowed = true;
    // check the character counts to make sure all (or all but one) are even
    for (int i = 0; i < counts.length; i++) {
      // if even number of characters, we keep going
      if (counts[i] % 2 == 0) continue;
      // else check to see if we've exhausted our allowance for a single odd value
      if (oddAllowed) {
        oddAllowed = false;
        continue;
      }
      return false;
    }
    return true;
  }

  // interactive command-line program
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);
    String input;
    while ((input = scanner.nextLine()).length() > 0) {
      System.out.println(PalindromePermutation.palindromepermutation(input));
    }
  }
}
