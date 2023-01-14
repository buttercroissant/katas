import java.util.ArrayList;

class CoffeeStoreTest {
    public static void main(String[] args) {
        CoffeeStore storeOne = new CoffeeStore();

        storeOne.setStoreName("starbucks");
        storeOne.setCoffeeBlend("java");
        storeOne.setCoffeeBlend("sumatra");
        storeOne.setCoffeeBlend("darkroast");

        ArrayList<String> storeOneAvailableBlends = storeOne.getCoffeeBlends();

        ArrayList<String> testBlends = new ArrayList<String>();
        testBlends.add("java");
        testBlends.add("sumatra");
        testBlends.add("darkroast");

        if(storeOneAvailableBlends.equals(testBlends)) {
            System.out.println("Passed (Object value check)");
        } else {
            System.out.println("Failed (Object value check)");
        }

        if(storeOneAvailableBlends == testBlends) {
            System.out.println("Passed (Object reference check)");
        } else {
            System.out.println("Failed (Object reference check)");
        }
         
    }
}
