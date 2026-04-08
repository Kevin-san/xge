
/// SQL query builder
pub struct SqlQuery {
    select: Vec<String>,
    from: Option<String>,
    where_clause: Option<String>,
    group_by: Vec<String>,
    order_by: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl SqlQuery {
    /// Creates a new SQL query builder
    pub fn new() -> Self {
        SqlQuery {
            select: Vec::new(),
            from: None,
            where_clause: None,
            group_by: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    /// Adds columns to select
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.select.extend(columns.iter().map(|c| c.to_string()));
        self
    }
    
    /// Sets the FROM clause
    pub fn from(mut self, table: &str) -> Self {
        self.from = Some(table.to_string());
        self
    }
    
    /// Sets the WHERE clause
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }
    
    /// Adds columns to GROUP BY
    pub fn group_by(mut self, columns: &[&str]) -> Self {
        self.group_by.extend(columns.iter().map(|c| c.to_string()));
        self
    }
    
    /// Adds columns to ORDER BY
    pub fn order_by(mut self, columns: &[&str]) -> Self {
        self.order_by.extend(columns.iter().map(|c| c.to_string()));
        self
    }
    
    /// Sets the LIMIT clause
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Sets the OFFSET clause
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
    
    /// Builds the SQL query string
    pub fn build(&self) -> String {
        let mut query = String::new();
        
        // SELECT clause
        if !self.select.is_empty() {
            query.push_str("SELECT ");
            query.push_str(&self.select.join(", "));
        } else {
            query.push_str("SELECT *");
        }
        
        // FROM clause
        if let Some(from) = &self.from {
            query.push_str(" FROM ");
            query.push_str(from);
        }
        
        // WHERE clause
        if let Some(where_clause) = &self.where_clause {
            query.push_str(" WHERE ");
            query.push_str(where_clause);
        }
        
        // GROUP BY clause
        if !self.group_by.is_empty() {
            query.push_str(" GROUP BY ");
            query.push_str(&self.group_by.join(", "));
        }
        
        // ORDER BY clause
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(&self.order_by.join(", "));
        }
        
        // LIMIT clause
        if let Some(limit) = self.limit {
            query.push_str(" LIMIT ");
            query.push_str(&limit.to_string());
        }
        
        // OFFSET clause
        if let Some(offset) = self.offset {
            query.push_str(" OFFSET ");
            query.push_str(&offset.to_string());
        }
        
        query.push(';');
        query
    }
}

