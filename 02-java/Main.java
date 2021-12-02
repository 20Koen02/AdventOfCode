import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

class Main {
    private static ArrayList<String> steps;

    public static int partOne() throws Exception {
        int horizontal = 0, depth = 0;

        for (String step : steps) {
            String[] instruction = step.split(" ");
            String direction = instruction[0];
            int distance = Integer.parseInt(instruction[1]);

            switch (direction) {
                case "forward":
                    horizontal += distance;
                    break;
                case "down":
                    depth += distance;
                    break;
                case "up":
                    depth -= distance;
                    break;
                default:
                    throw new Exception("Invalid direction: " + direction);
            }
        }

        return horizontal * depth;
    }

    public static int partTwo() throws Exception {
        int horizontal = 0, depth = 0, aim = 0;

        for (String step : steps) {
            String[] instruction = step.split(" ");
            String direction = instruction[0];
            int distance = Integer.parseInt(instruction[1]);

            switch (direction) {
                case "forward":
                    horizontal += distance;
                    depth += distance * aim;
                    break;
                case "down":
                    aim += distance;
                    break;
                case "up":
                    aim -= distance;
                    break;
                default:
                    throw new Exception("Invalid direction: " + direction);
            }
        }

        return horizontal * depth;
    }

    public static void main(String[] args) throws Exception {
        List<String> stepsList = Files.readAllLines(Paths.get("02-java/in.txt"));
        steps = new ArrayList<String>(stepsList);

        System.out.println("Day 2 part one: " + String.valueOf(partOne()));
        System.out.println("Day 2 part two: " + String.valueOf(partTwo()));
    }
}