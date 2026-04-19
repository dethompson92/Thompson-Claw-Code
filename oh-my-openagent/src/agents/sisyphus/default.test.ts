/// <reference types="bun-types" />

import { describe, expect, it } from "bun:test"
import { buildTaskManagementSection } from "./default"

describe("buildTaskManagementSection", () => {
  it("renders task-tool guidance when the task system is enabled", () => {
    const section = buildTaskManagementSection(true)

    expect(section).toContain("## Task Management (CRITICAL)")
    expect(section).toContain("`TaskCreate`")
    expect(section).toContain("`TaskUpdate(status=\"in_progress\")`")
    expect(section).not.toContain("todowrite")
  })

  it("renders todo guidance when the task system is disabled", () => {
    const section = buildTaskManagementSection(false)

    expect(section).toContain("## Todo Management (CRITICAL)")
    expect(section).toContain("`todowrite`")
    expect(section).toContain("Mark `completed` IMMEDIATELY")
    expect(section).not.toContain("`TaskCreate`")
  })
})
