//
//  AutomergeRSBackend.h
//  AutomergeRSBackend
//
//  Created by Lukas Schmidt on 05.06.20.
//  Copyright © 2020 Automerge. All rights reserved.
//

#import <Foundation/Foundation.h>

//! Project version number for AutomergeRSBackend.
FOUNDATION_EXPORT double AutomergeRSBackendVersionNumber;

//! Project version string for AutomergeRSBackend.
FOUNDATION_EXPORT const unsigned char AutomergeRSBackendVersionString[];

// In this header, you should import all the public headers of your framework using statements like #import <AutomergeRSBackend/PublicHeader.h>

#include <stdint.h>
#include <stdbool.h>

typedef struct Backend Backend;

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
intptr_t automerge_apply_changes(Backend *backend);

/**
 * # Safety
 * This must me called with a valid backend pointer
 * request must be a valid pointer pointing to a cstring
 */
intptr_t automerge_apply_local_change(Backend *backend, const char *request);

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
Backend *automerge_clone(Backend *backend);

/**
 * # Safety
 * This must me called with a valid pointer to a change and the correct len
 */
intptr_t automerge_decode_change(Backend *backend, uintptr_t len, const uint8_t *change);

/**
 * # Safety
 * This must me called with a valid pointer a json string of a change
 */
intptr_t automerge_encode_change(Backend *backend, const char *change);

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
void automerge_free(Backend *backend);

/**
 * # Safety
 * This must me called with a valid backend pointer
 * binary must be a valid pointer to len bytes
 */
intptr_t automerge_get_changes(Backend *backend, uintptr_t len, const uint8_t *binary);

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
intptr_t automerge_get_changes_for_actor(Backend *backend, const char *actor);

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
intptr_t automerge_get_missing_deps(Backend *backend);

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
intptr_t automerge_get_patch(Backend *backend);

Backend *automerge_init(void);

/**
 * # Safety
 * data pointer must be a valid pointer to len bytes
 */
Backend *automerge_load(uintptr_t len, const uint8_t *data);

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
intptr_t automerge_load_changes(Backend *backend);

/**
 * # Safety
 *
 * This must me called with a valid backend pointer
 * the buffer must be a valid pointer pointing to at least as much space as was
 * required by the previous binary result call
 */
intptr_t automerge_read_binary(Backend *backend, uint8_t *buffer);

/**
 * # Safety
 * This must me called with a valid backend pointer
 * and buffer must be a valid pointer of at least the number of bytes returned by the previous
 * call that generated a json result
 */
intptr_t automerge_read_json(Backend *backend, char *buffer);

/**
 * # Safety
 * This must me called with a valid backend pointer
 */
intptr_t automerge_save(Backend *backend);

/**
 * # Safety
 * This must me called with a valid backend pointer
 * change must point to a valid memory location with at least len bytes
 */
void automerge_write_change(Backend *backend, uintptr_t len, const uint8_t *change);

