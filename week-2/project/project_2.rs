fn main() {
// For Toshiba products:
let qty_1:f64 = 2.00;
let amount_1:f64 = 450_000.00;

// For Mac Products:
let qty_2:f64 = 1.00;
let amount_2:f64 = 1_500_000.00;

// For HP Products:
let qty_3:f64 = 3.00;
let amount_3:f64 = 750_000.00;

//For Dell Products:
let qty_4:f64 = 3.00;
let amount_4:f64 = 2_850_000.00;

//For Acer Products:
let qty_5:f64 = 1.00;
let amount_5:f64 = 250_000.00;

// To find the sum of the quantity of goods sold in the sales record.
let sum = (qty_1 * amount_1) + (qty_2 * amount_2) + (qty_3 * amount_3) + (qty_4 * amount_4) + (qty_5 * amount_5);
println!("The total amount of all the products is N{}", sum);

// To find the average of the total quantity of goods sold in the sales record.
let qty_total = qty_1 + qty_2 + qty_3 + qty_4 + qty_5;
let avg_amount = sum / qty_total;
println!("The avg_amount is {}", avg_amount);

}