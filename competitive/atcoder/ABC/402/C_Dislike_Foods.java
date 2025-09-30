import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int numFood = sc.nextInt();
        int numMenu = sc.nextInt();

        // 食材ごとに含まれるメニューIDのリスト
        List<List<Integer>> foodToMenus = new ArrayList<>(numFood);
        for (int i = 0; i < numFood; i++) {
            foodToMenus.add(new ArrayList<>());
        }

        // 各メニューに残っている必要食材数
        int[] remaining = new int[numMenu];

        for (int menuId = 0; menuId < numMenu; menuId++) {
            int numMenuFood = sc.nextInt();
            remaining[menuId] = numMenuFood;

            for (int j = 0; j < numMenuFood; j++) {
                int foodIndex = sc.nextInt() - 1; // 0-based
                foodToMenus.get(foodIndex).add(menuId);
            }
        }

        int completedMenus = 0;
        for (int i = 0; i < numFood; i++) {
            int food = sc.nextInt() - 1;

            for (int menuId : foodToMenus.get(food)) {
                remaining[menuId]--;
                if (remaining[menuId] == 0) {
                    completedMenus++;
                }
            }

            System.out.println(completedMenus);
        }

        sc.close();
    }
}
