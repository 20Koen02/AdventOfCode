import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

class Main {
    private static ArrayList<String> steps;

    public static int[] solve() {
        int horizontal = 0, depth = 0, aim = 0;

        for (String step : steps) {
            String[] instruction = step.split(" ");
            String direction = instruction[0];
            int distance = Integer.parseInt(instruction[1]);

            switch (direction) {
                case "forward" -> {
                    horizontal += distance;
                    depth += distance * aim;
                }
                case "down" -> aim += distance;
                case "up" -> aim -= distance;
            }

        }

        return new int[] { horizontal * aim, horizontal * depth };
    }

    public static void main(String[] args) throws Exception {
        List<String> stepsList = Files.readAllLines(Paths.get("02-java/in.txt"));
        steps = new ArrayList<String>(stepsList);

        int[] solution = solve();

        System.out.println("Day 2 part one: " + String.valueOf(solution[0]));
        System.out.println("Day 2 part two: " + String.valueOf(solution[1]));
    }
}