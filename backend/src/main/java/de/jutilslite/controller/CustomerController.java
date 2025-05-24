package de.jutilslite.controller;

import de.jutilslite.entity.Customer;
import de.jutilslite.repository.CustomerRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/customers")
public class CustomerController {

    @Autowired
    private CustomerRepository customerRepository;

    @GetMapping
    public List<Customer> findAll()
    {
        return (List<Customer>) customerRepository.findAll();
    }

    @PostMapping
    public Customer create(@RequestBody Customer customer)
    {
        return customerRepository.save(customer);
    }

}
