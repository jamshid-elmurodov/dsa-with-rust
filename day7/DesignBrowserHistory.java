class BrowserHistory {
    private class Node {
        String val;
        Node next;
        Node prev;

        Node(String url){
            this.val = url;
        }
    }

    private Node curr;

    public BrowserHistory(String url) {
        curr = new Node(url);
    }
    
    public void visit(String url) {
        curr.next = new Node(url);
        curr.next.prev = curr;
        curr = curr.next;   
    }
    
    public String back(int steps) {
        while(curr.prev != null && steps-- > 0){
            curr = curr.prev;
        } 
        
        return curr.val;
    }
    
    public String forward(int steps) {
        while(curr.next != null && steps-- > 0){
            curr = curr.next;
        }

        return curr.val; 
    }  
}