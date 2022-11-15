import java.util.ArrayList;

class CoffeeStore {
    private String storeName;
    private ArrayList<String> coffeeBlends = new ArrayList<String>(); 

    public String getStoreName() {
        return storeName;
    }

    public void setStoreName(String _storeName) {
        storeName = _storeName;
    }

    public ArrayList<String> getCoffeeBlends() {
        return coffeeBlends;
    }

    public void setCoffeeBlend(String _coffeeBlend) {
        coffeeBlends.add(_coffeeBlend);
    }
}
