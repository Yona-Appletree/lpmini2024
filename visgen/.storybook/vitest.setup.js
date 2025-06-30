"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var vitest_1 = require("vitest");
var react_1 = require("@storybook/react");
var projectAnnotations = require("./preview");
// This is an important step to apply the right configuration when testing your stories.
// More info at: https://storybook.js.org/docs/api/portable-stories/portable-stories-vitest#setprojectannotations
var project = (0, react_1.setProjectAnnotations)([projectAnnotations]);
(0, vitest_1.beforeAll)(project.beforeAll);
