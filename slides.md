
# Agenda

- Introduction
  - Concurrency vs parallelism
  - Why bother with concurrency?
  - Diminishing returns
    - Amdahl's Law
    - First example proving the point

---

# Introduction: Parallelism vs Concurrency



**Parallelism:**


--

.middle["*A condition that arises when at least two threads are **executing** simultaneously."*]

--


**Concurrency:**

--


.middle["A condition that exists when at least two threads are **making progress**. A more generalized form of parallelism that can include time-slicing as a form of virtual parallelism."]


---


# Amdahl's Law

</br>
</br>
# $$T = { 1 \over { s + p \over N }}$$

---


# Amdahl's Law

.center[![amdahls_law](amdahls_law_scale.png)]

---

class: .middle

# Example


---
