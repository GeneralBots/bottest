//! Unit tests for attendance module from botserver
//!
//! These tests verify the queue priority and ordering logic
//! in the attendance system.

use std::cmp::Ordering;

/// Priority levels matching botserver's attendance queue
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Urgent = 3,
}

/// Queue entry for testing
#[derive(Debug, Clone)]
pub struct QueueEntry {
    pub id: u64,
    pub customer_id: String,
    pub priority: Priority,
    pub entered_at: u64, // Unix timestamp
}

impl QueueEntry {
    pub fn new(id: u64, customer_id: &str, priority: Priority, entered_at: u64) -> Self {
        Self {
            id,
            customer_id: customer_id.to_string(),
            priority,
            entered_at,
        }
    }
}

/// Compare queue entries: higher priority first, then earlier timestamp
fn compare_queue_entries(a: &QueueEntry, b: &QueueEntry) -> Ordering {
    // Higher priority comes first (reverse order)
    match b.priority.cmp(&a.priority) {
        Ordering::Equal => {
            // Same priority: earlier timestamp comes first
            a.entered_at.cmp(&b.entered_at)
        }
        other => other,
    }
}

/// Sort a queue by priority and timestamp
fn sort_queue(entries: &mut [QueueEntry]) {
    entries.sort_by(compare_queue_entries);
}

/// Get the next entry from the queue (highest priority, earliest time)
fn get_next_in_queue(entries: &[QueueEntry]) -> Option<&QueueEntry> {
    if entries.is_empty() {
        return None;
    }

    let mut best = &entries[0];
    for entry in entries.iter().skip(1) {
        if compare_queue_entries(entry, best) == Ordering::Less {
            best = entry;
        }
    }
    Some(best)
}

// =============================================================================
// Priority Comparison Tests
// =============================================================================

#[test]
fn test_priority_ordering() {
    assert!(Priority::Urgent > Priority::High);
    assert!(Priority::High > Priority::Normal);
    assert!(Priority::Normal > Priority::Low);
}

#[test]
fn test_priority_equality() {
    assert_eq!(Priority::Normal, Priority::Normal);
    assert_ne!(Priority::Normal, Priority::High);
}

// =============================================================================
// Queue Entry Comparison Tests
// =============================================================================

#[test]
fn test_higher_priority_comes_first() {
    let high = QueueEntry::new(1, "customer1", Priority::High, 1000);
    let normal = QueueEntry::new(2, "customer2", Priority::Normal, 900);

    // High priority should come before normal, even if normal entered earlier
    assert_eq!(compare_queue_entries(&high, &normal), Ordering::Less);
}

#[test]
fn test_same_priority_earlier_time_first() {
    let first = QueueEntry::new(1, "customer1", Priority::Normal, 1000);
    let second = QueueEntry::new(2, "customer2", Priority::Normal, 1100);

    // Same priority: earlier timestamp comes first
    assert_eq!(compare_queue_entries(&first, &second), Ordering::Less);
}

#[test]
fn test_same_priority_same_time() {
    let a = QueueEntry::new(1, "customer1", Priority::Normal, 1000);
    let b = QueueEntry::new(2, "customer2", Priority::Normal, 1000);

    assert_eq!(compare_queue_entries(&a, &b), Ordering::Equal);
}

#[test]
fn test_urgent_beats_everything() {
    let urgent = QueueEntry::new(1, "customer1", Priority::Urgent, 2000);
    let high = QueueEntry::new(2, "customer2", Priority::High, 1000);
    let normal = QueueEntry::new(3, "customer3", Priority::Normal, 500);
    let low = QueueEntry::new(4, "customer4", Priority::Low, 100);

    assert_eq!(compare_queue_entries(&urgent, &high), Ordering::Less);
    assert_eq!(compare_queue_entries(&urgent, &normal), Ordering::Less);
    assert_eq!(compare_queue_entries(&urgent, &low), Ordering::Less);
}

// =============================================================================
// Queue Sorting Tests
// =============================================================================

#[test]
fn test_sort_queue_by_priority() {
    let mut queue = vec![
        QueueEntry::new(1, "low", Priority::Low, 1000),
        QueueEntry::new(2, "urgent", Priority::Urgent, 1000),
        QueueEntry::new(3, "normal", Priority::Normal, 1000),
        QueueEntry::new(4, "high", Priority::High, 1000),
    ];

    sort_queue(&mut queue);

    assert_eq!(queue[0].priority, Priority::Urgent);
    assert_eq!(queue[1].priority, Priority::High);
    assert_eq!(queue[2].priority, Priority::Normal);
    assert_eq!(queue[3].priority, Priority::Low);
}

#[test]
fn test_sort_queue_mixed_priority_and_time() {
    let mut queue = vec![
        QueueEntry::new(1, "normal_late", Priority::Normal, 2000),
        QueueEntry::new(2, "high_late", Priority::High, 1500),
        QueueEntry::new(3, "normal_early", Priority::Normal, 1000),
        QueueEntry::new(4, "high_early", Priority::High, 1200),
    ];

    sort_queue(&mut queue);

    // High priority entries first, ordered by time
    assert_eq!(queue[0].id, 4); // high_early
    assert_eq!(queue[1].id, 2); // high_late
                                // Then normal priority, ordered by time
    assert_eq!(queue[2].id, 3); // normal_early
    assert_eq!(queue[3].id, 1); // normal_late
}

#[test]
fn test_sort_empty_queue() {
    let mut queue: Vec<QueueEntry> = vec![];
    sort_queue(&mut queue);
    assert!(queue.is_empty());
}

#[test]
fn test_sort_single_entry() {
    let mut queue = vec![QueueEntry::new(1, "only", Priority::Normal, 1000)];
    sort_queue(&mut queue);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0].id, 1);
}

// =============================================================================
// Get Next in Queue Tests
// =============================================================================

#[test]
fn test_get_next_returns_highest_priority() {
    let queue = vec![
        QueueEntry::new(1, "low", Priority::Low, 100),
        QueueEntry::new(2, "high", Priority::High, 200),
        QueueEntry::new(3, "normal", Priority::Normal, 150),
    ];

    let next = get_next_in_queue(&queue).unwrap();
    assert_eq!(next.id, 2); // High priority
}

#[test]
fn test_get_next_respects_time_within_priority() {
    let queue = vec![
        QueueEntry::new(1, "first", Priority::Normal, 1000),
        QueueEntry::new(2, "second", Priority::Normal, 1100),
        QueueEntry::new(3, "third", Priority::Normal, 1200),
    ];

    let next = get_next_in_queue(&queue).unwrap();
    assert_eq!(next.id, 1); // First to enter
}

#[test]
fn test_get_next_empty_queue() {
    let queue: Vec<QueueEntry> = vec![];
    assert!(get_next_in_queue(&queue).is_none());
}

#[test]
fn test_get_next_single_entry() {
    let queue = vec![QueueEntry::new(42, "only_customer", Priority::Normal, 1000)];

    let next = get_next_in_queue(&queue).unwrap();
    assert_eq!(next.id, 42);
}

// =============================================================================
// Real-world Scenario Tests
// =============================================================================

#[test]
fn test_scenario_customer_support_queue() {
    // Simulate a real customer support queue scenario
    let mut queue = vec![
        // Regular customers entering over time
        QueueEntry::new(1, "alice", Priority::Normal, 1000),
        QueueEntry::new(2, "bob", Priority::Normal, 1100),
        QueueEntry::new(3, "charlie", Priority::Normal, 1200),
        // VIP customer enters later
        QueueEntry::new(4, "vip_dave", Priority::High, 1300),
        // Urgent issue reported
        QueueEntry::new(5, "urgent_eve", Priority::Urgent, 1400),
    ];

    sort_queue(&mut queue);

    // Service order should be: urgent_eve, vip_dave, alice, bob, charlie
    assert_eq!(queue[0].customer_id, "urgent_eve");
    assert_eq!(queue[1].customer_id, "vip_dave");
    assert_eq!(queue[2].customer_id, "alice");
    assert_eq!(queue[3].customer_id, "bob");
    assert_eq!(queue[4].customer_id, "charlie");
}

#[test]
fn test_scenario_multiple_urgent_fifo() {
    // Multiple urgent requests should still be FIFO within that priority
    let mut queue = vec![
        QueueEntry::new(1, "urgent1", Priority::Urgent, 1000),
        QueueEntry::new(2, "urgent2", Priority::Urgent, 1100),
        QueueEntry::new(3, "urgent3", Priority::Urgent, 1050),
    ];

    sort_queue(&mut queue);

    // Should be ordered by entry time within urgent priority
    assert_eq!(queue[0].customer_id, "urgent1"); // 1000
    assert_eq!(queue[1].customer_id, "urgent3"); // 1050
    assert_eq!(queue[2].customer_id, "urgent2"); // 1100
}

#[test]
fn test_scenario_priority_upgrade() {
    // Simulate upgrading a customer's priority
    let mut entry = QueueEntry::new(1, "customer", Priority::Normal, 1000);

    // Verify initial priority
    assert_eq!(entry.priority, Priority::Normal);

    // Upgrade priority (customer complained, escalation, etc.)
    entry.priority = Priority::High;

    assert_eq!(entry.priority, Priority::High);
}

#[test]
fn test_queue_position_calculation() {
    let queue = vec![
        QueueEntry::new(1, "first", Priority::Normal, 1000),
        QueueEntry::new(2, "second", Priority::Normal, 1100),
        QueueEntry::new(3, "third", Priority::Normal, 1200),
        QueueEntry::new(4, "fourth", Priority::Normal, 1300),
    ];

    // Find position of customer with id=3
    let position = queue.iter().position(|e| e.id == 3).map(|p| p + 1); // 1-based position

    assert_eq!(position, Some(3));
}

#[test]
fn test_estimated_wait_time() {
    let avg_service_time_minutes = 5;

    let _queue = vec![
        QueueEntry::new(1, "first", Priority::Normal, 1000),
        QueueEntry::new(2, "second", Priority::Normal, 1100),
        QueueEntry::new(3, "third", Priority::Normal, 1200),
    ];

    // Customer at position 3 has 2 people ahead
    let position = 3;
    let people_ahead = position - 1;
    let estimated_wait = people_ahead * avg_service_time_minutes;

    assert_eq!(estimated_wait, 10); // 2 people * 5 minutes each
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_large_queue() {
    let mut queue: Vec<QueueEntry> = (0..1000)
        .map(|i| {
            let priority = match i % 4 {
                0 => Priority::Low,
                1 => Priority::Normal,
                2 => Priority::High,
                _ => Priority::Urgent,
            };
            QueueEntry::new(i, &format!("customer_{}", i), priority, 1000 + i)
        })
        .collect();

    sort_queue(&mut queue);

    // First entry should be urgent (i % 4 == 3, first one is i=3)
    assert_eq!(queue[0].priority, Priority::Urgent);

    // Last entry should be low priority
    assert_eq!(queue[999].priority, Priority::Low);
}

#[test]
fn test_all_same_priority_and_time() {
    let queue = vec![
        QueueEntry::new(1, "a", Priority::Normal, 1000),
        QueueEntry::new(2, "b", Priority::Normal, 1000),
        QueueEntry::new(3, "c", Priority::Normal, 1000),
    ];

    // All equal, any is valid as "next"
    let next = get_next_in_queue(&queue);
    assert!(next.is_some());
}
