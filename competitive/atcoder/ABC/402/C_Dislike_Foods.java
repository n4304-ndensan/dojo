import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int num_food = sc.nextInt();
        int num_Menu = sc.nextInt();

        List<List<Integer>> menus = new ArrayList<>();
        for (int i = 0; i < num_Menu; i++ ) {
            int num_menu_food = sc.nextInt();
            List<Integer> tmp_menus = new ArrayList<>();
            for (int j = 0; j < num_menu_food; j++) {
                tmp_menus.add(sc.nextInt());
            }
            menus.add(tmp_menus);
        }


        int eatable_menus = 0;


        for (int i = 0; i < num_food; i++) {
            int food = sc.nextInt();

            Iterator<List<Integer>> it = menus.iterator();
            while (it.hasNext()) {
                List<Integer> menu = it.next();

                if (menu.contains(food)) {
                    menu.remove(Integer.valueOf(food));
                }

                if (menu.isEmpty()) {
                    eatable_menus++;
                    it.remove();
                }
            }

            System.out.println(eatable_menus);
        }
    }
}
