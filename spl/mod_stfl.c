/*
 *  STFL - The Structured Terminal Forms Language/Library
 *  Copyright (C) 2006  Clifford Wolf <clifford@clifford.at>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 *  mod_stfl.c: STFL bindings for SPL
 */

/**
 * STFL module
 *
 * This module provides bindings to the Structured Terminal Forms
 * Language/Library (STFL).
 */

#define _GNU_SOURCE

#include "stfl.h"

#include <spl.h>
#include <stdlib.h>

extern void SPL_ABI(spl_mod_stfl_init)(struct spl_vm *vm, struct spl_module *mod, int restore);
extern void SPL_ABI(spl_mod_stfl_done)(struct spl_vm *vm, struct spl_module *mod);

static void handler_stfl_form_node(struct spl_task *task, struct spl_vm *vm, struct spl_node *node, struct spl_hnode_args *args, void *data)
{
	if (args->action == SPL_HNODE_ACTION_PUT) {
		if (node->hnode_data)
			stfl_free((struct stfl_form *)node->hnode_data);
	}
}

static struct stfl_form *clib_get_stfl_form(struct spl_task *task)
{
	struct spl_node *n = spl_cleanup(task, spl_clib_get_node(task));
	struct stfl_form *f = n ? n->hnode_data : 0;

	if (!f || !n->hnode_name || strcmp(n->hnode_name, "stfl_form")) {
		spl_report(SPL_REPORT_RUNTIME, task,
				"Expected a STFL-Form as 1st argument!\n");
		return 0;
	}

	return f;
}

static struct spl_node *spl_new_nullable_ascii(const char *text)
{
	return text ? SPL_NEW_STRING(spl_utf8_import(text, "ascii")) : spl_get(0);
}

/**
 * Quote a string to be used in STFL code.
 *
 * This function is designed to be used with the encoding/quoting operator (::).
 */
// builtin encode_stfl(text)

/**
 * Parse an STFL description text and return the form handler
 */
// builtin stfl_create(text)
static struct spl_node *handler_stfl_create(struct spl_task *task, void *data)
{
	struct spl_node *n = SPL_NEW_STRING_DUP("STFL Form");
	n->hnode_name = strdup("stfl_form");
	n->hnode_data = stfl_create(spl_clib_get_string(task));;
	return n;
}

/**
 * Display the form and process one input character
 */
// builtin stfl_run(form, timeout)
static struct spl_node *handler_stfl_run(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	return f ? spl_new_nullable_ascii(stfl_run(f, spl_clib_get_int(task))) : 0;
}

/**
 * Return to standard text mode
 */
// builtin stfl_return()
static struct spl_node *handler_stfl_return(struct spl_task *task, void *data)
{
	stfl_return();
	return 0;
}

/**
 * Get the value of an STFL variable
 */
// builtin stfl_get(form, name)
static struct spl_node *handler_stfl_get(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	return f ? spl_new_nullable_ascii(stfl_get(f, spl_clib_get_string(task))) : 0;
}

/**
 * Set an STFL variable
 */
// builtin stfl_set(form, name, value)
static struct spl_node *handler_stfl_set(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	char *name = spl_clib_get_string(task);
	char *value = spl_clib_get_string(task);
	stfl_set(f, name, value);
	return 0;
}

/**
 * Get the name of the widget currently having the focus
 */
// builtin stfl_get_focus(form)
static struct spl_node *handler_stfl_get_focus(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	return f ? spl_new_nullable_ascii(stfl_get_focus(f)) : 0;
}

/**
 * Set the focus to the specified widget
 */
// builtin stfl_set_focus(form, name)
static struct spl_node *handler_stfl_set_focus(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	stfl_set_focus(f, spl_clib_get_string(task));
	return 0;
}

/**
 * Quote a string to be used in STFL code
 */
// builtin stfl_quote(text)
static struct spl_node *handler_stfl_quote(struct spl_task *task, void *data)
{
	const char *text = stfl_quote(spl_clib_get_string(task));
	struct spl_node *n = spl_new_nullable_ascii(text);
	return n;
}

/**
 * Dump the STFL Code for this form
 */
// builtin stfl_dump(form, name, prefix, focus)
static struct spl_node *handler_stfl_dump(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	char *name = spl_clib_get_string(task);
	char *prefix = spl_clib_get_string(task);
	int focus = spl_clib_get_int(task);
	const char *text = stfl_dump(f, name, prefix, focus);
	struct spl_node *n = spl_new_nullable_ascii(text);
	return n;
}

/**
 * Import STFL code to an existing form
 */
// builtin stfl_import(form, name, mode, text)
static struct spl_node *handler_stfl_import(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	char *name = spl_clib_get_string(task);
	char *mode = spl_clib_get_string(task);
	char *text = spl_clib_get_string(task);
	stfl_import(f, name, mode, text);
	return 0;
}

/**
 * Lookup widgets in the form using a path and optionally assign a new name
 */
// builtin stfl_lookup(form, path, newname)
static struct spl_node *handler_stfl_lookup(struct spl_task *task, void *data)
{
	struct stfl_form *f = clib_get_stfl_form(task);
	char *path = spl_clib_get_string(task);
	char *newname = spl_clib_get_string(task);
	return spl_new_nullable_ascii(stfl_lookup(f, path, newname));
}

/**
 * Return error message of last stfl call or undef.
 */
// builtin stfl_error()
static struct spl_node *handler_stfl_error(struct spl_task *task, void *data)
{
	return spl_new_nullable_ascii(stfl_error());
}

/**
 * Set error handling algorithm.
 */
// builtin stfl_error_action(mode)
static struct spl_node *handler_stfl_error_action(struct spl_task *task, void *data)
{
	char *mode = spl_clib_get_string(task);
	stfl_error_action(mode);
	return 0;
}

void SPL_ABI(spl_mod_stfl_init)(struct spl_vm *vm, struct spl_module *mod, int restore)
{
	spl_hnode_reg(vm, "stfl_form", handler_stfl_form_node, 0);

	spl_clib_reg(vm, "stfl_create", handler_stfl_create, 0);

	spl_clib_reg(vm, "stfl_run", handler_stfl_run, 0);
	spl_clib_reg(vm, "stfl_return", handler_stfl_return, 0);

	spl_clib_reg(vm, "stfl_get", handler_stfl_get, 0);
	spl_clib_reg(vm, "stfl_set", handler_stfl_set, 0);

	spl_clib_reg(vm, "stfl_get_focus", handler_stfl_get_focus, 0);
	spl_clib_reg(vm, "stfl_set_focus", handler_stfl_set_focus, 0);

	spl_clib_reg(vm, "encode_stfl", handler_stfl_quote, 0);
	spl_clib_reg(vm, "stfl_quote", handler_stfl_quote, 0);

	spl_clib_reg(vm, "stfl_dump", handler_stfl_dump, 0);
	spl_clib_reg(vm, "stfl_import", handler_stfl_import, 0);
	spl_clib_reg(vm, "stfl_lookup", handler_stfl_lookup, 0);

	spl_clib_reg(vm, "stfl_error", handler_stfl_error, 0);
	spl_clib_reg(vm, "stfl_error_action", handler_stfl_error_action, 0);
}

void SPL_ABI(spl_mod_stfl_done)(struct spl_vm *vm, struct spl_module *mod)
{
	stfl_return();
}

