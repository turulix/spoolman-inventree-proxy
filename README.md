# Spoolman-InvenTree Proxy

A lightweight Rust proxy that **mimics the Spoolman API**, enabling systems that support Spoolman to send filament-related data—including usage updates—to InvenTree, without running an actual Spoolman instance.

---

##  Overview

This proxy **pretends to be Spoolman**, exposing compatible endpoints (such as for weight updates, spool selection, etc.) so that applications like Moonraker, Klipper, or other systems can send requests as if they were talking to Spoolman. The proxy then maps or transforms these incoming requests to update filament inventory in InvenTree.

---

##  Features

- **API emulation**: Implements relevant portions of the Spoolman API surface (e.g., webhook, weight reporting, spool activation).
- **Seamless integration**: Works transparently with tools expecting to connect to Spoolman (e.g., Moonraker).
- **InvenTree updates**: Translates and sends corresponding inventory updates to InvenTree via its REST API.
- **Rust-powered**: Efficient, safe, and easy to deploy.

---

##  Installation & Setup

TODO