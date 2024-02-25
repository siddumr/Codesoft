/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
 package task2;import java.util.Scanner;
 public class Task2
 {
     public static void main(String[] args)
     {
         Scanner scanner = new Scanner(System.in);
 
         System.out.print("Enter the Number of Subjects: ");
         int numSubjects = scanner.nextInt();
 
         int[] marks = new int[numSubjects];
         int totalMarks = 0;
         
         for (int i = 0; i < numSubjects; i++)
         {
             System.out.print("Enter marks obtained in Subject " + (i + 1) + ": ");
             marks[i] = scanner.nextInt();
             totalMarks += marks[i];
         }
         double averagePercentage = (double) totalMarks / (numSubjects * 100) * 100;
 
         System.out.println(" Result:");
         System.out.println("Your Total Marks: " + totalMarks);
         System.out.println("Average Percentage: " + averagePercentage + "%");
         String grade = calculateGrade(averagePercentage);
         System.out.println("Grade: " + grade);
         scanner.close();
     }
 
     public static String calculateGrade(double percentage)
     {
         if (percentage >= 90) 
         {
             return "0";
         }
         else if (percentage >= 80)
         {
             return "A";
         }
         else if (percentage >= 70)
         {
             return "B";
         }
         else if (percentage >= 60)
         {
             return "C";
         } 
         else if (percentage >= 50) 
         {
             return "D";
         } 
         else 
         {
             return "F";
         }
     }
 }
 
 