#[derive(Debug)]
pub struct Company {
    pub name: String,
    pub cash_balance: f64,
    pub services: Vec<Service>,
}

#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub employee_type: EmployeeType,
    pub salary: f64,
}

#[derive(Debug)]
pub enum EmployeeType {
    Manager,
    Engineer,
    Salesperson,
    Worker,
    Technician,
}

#[derive(Debug)]
pub enum ServiceType {
    Logistics,
    Marketing,
    Development,
    ITDepartment,
    Sales,
    Purchasing,
    Administration,
    Production,
}

#[derive(Debug)]
pub struct Service {
    pub service_type: ServiceType,
    pub employees: Vec<Employee>,
}

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: f64,
}

